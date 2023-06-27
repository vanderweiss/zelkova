// Main abstraction layer between wgpu and toolkit essentials

use {
    bytemuck::{self, NoUninit},
    pollster,
    std::{
        borrow::Cow,
        ptr,
        sync::{LazyLock, Mutex},
    },
    wgpu::{self, util::DeviceExt},
};

mod _sealed {
    pub trait Sealed {}
}

pub trait Component: _sealed::Sealed + NoUninit {}

macro_rules! impl_component {
    ($($ty:ident)*) => {$(
        impl Component for $ty {}
        impl _sealed::Sealed for $ty {}
    )*}
}

// Valid types to operate on buffers
impl_component! {
    u16 u32 u64
    i16 i32 i64
    f32 f64
}

/* Still unsure whether to keep this or not
pub(crate) trait Resource: _sealed::Sealed {
    fn label() -> Label<'static>;
}

macro_rules! impl_resource {
    ($($resource:ident, $alias:literal)*) => {$(
        impl Resource for wgpu::$resource {
            fn label() -> Label<'static> {
                Some("Zelkova owned $alias.")
            }
        }
        impl _sealed::Sealed for wgpu::$resource {}
    )*}
}

impl_resource! {
    Buffer, "buffer"
    CommandEncoder, "command encoder"
    ComputePipeline, "compute pipeline"
}

*/

/// Generic label for every allocated zelkova-owned resource.
static label: Option<&'static str> = Some("Zelkova owned resource");

// Core interface to handle wgpu internals
pub(crate) struct Handler {
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    encoder: wgpu::CommandEncoder,
    queue: wgpu::Queue,
}

impl Handler {
    #[must_use]
    fn _request() -> Result<Self, wgpu::Error> {
        pollster::block_on(async move {
            let adapter = wgpu::Instance::default()
                .request_adapter(&wgpu::RequestAdapterOptions::default())
                .await
                .unwrap();

            // known issue with Lavapipe
            if adapter.get_info().vendor == 0x10005 {
                panic!();
            }

            let (device, queue) = adapter
                .request_device(&wgpu::DeviceDescriptor::default(), None)
                .await
                .unwrap();

            let encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

            let handler = Self {
                adapter,
                device,
                encoder,
                queue,
            };

            Ok(handler)
        })
    }

    #[must_use]
    pub fn request() -> Result<*mut Self, wgpu::Error> {
        static _handler: LazyLock<Mutex<Handler>> =
            LazyLock::new(|| Mutex::new(Handler::_request().expect("Failed to connect to GPU")));

        Ok(_handler
            .lock()
            .as_deref_mut()
            .map(|r| ptr::from_mut(r))
            .unwrap())
    }

    pub fn start_pass(&mut self) -> Result<wgpu::ComputePass, wgpu::Error> {
        let pass = self
            .encoder
            .begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });

        Ok(pass)
    }

    pub fn load_module(
        &self,
        module: Cow<'_, str>,
    ) -> Result<(wgpu::ShaderModule, wgpu::ComputePipeline), wgpu::Error> {
        let module = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label,
                source: wgpu::ShaderSource::Wgsl(module),
            });

        let pipeline = self
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label,
                layout: None,
                module: &module,
                entry_point: "main",
            });

        Ok((module, pipeline))
    }

    pub fn alloc_buffer_init(&self, contents: &[u8]) -> Result<wgpu::Buffer, wgpu::Error> {
        let buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label,
                contents,
                usage: wgpu::BufferUsages::MAP_READ,
            });

        Ok(buffer)
    }

    pub fn alloc_uninit_buffer(&self) {}

    pub fn join(
        &self,
        entries: &[wgpu::BindGroupEntry],
        meta: &[wgpu::BindGroupLayoutEntry],
    ) -> Result<wgpu::BindGroup, wgpu::Error> {
        let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label,
            layout: &self
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label,
                    entries: meta,
                }),
            entries,
        });

        Ok(group)
    }
}

pub(crate) struct Buffer {
    /// Actual internal wgpu buffer.    
    _buffer: wgpu::Buffer,
    /// Binding assigned at runtime.
    pub binding: u32,
}

impl Buffer {
    pub fn bind<C: Component>(content: &[C], binding: u32) -> Result<Self, wgpu::Error> {
        let _buffer = unsafe {
            Handler::request()?
                .as_ref()
                .unwrap()
                .alloc_buffer_init(bytemuck::cast_slice::<C, u8>(content))?
        };

        let entry = Self { _buffer, binding };

        Ok(entry)
    }

    pub fn group(&self) -> Result<wgpu::BindGroupEntry, wgpu::Error> {
        let _group = wgpu::BindGroupEntry {
            binding: self.binding,
            resource: self._buffer.as_entire_binding(),
        };

        Ok(_group)
    }

    pub fn layout(&self) -> Result<wgpu::BindGroupLayoutEntry, wgpu::Error> {
        let _layout = wgpu::BindGroupLayoutEntry {
            binding: self.binding,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };

        Ok(_layout)
    }

    #[inline]
    pub fn free(&self) {
        drop(self._buffer.slice(..).get_mapped_range());
    }

    #[inline]
    pub fn get(&self) -> &wgpu::Buffer {
        &self._buffer
    }

    #[inline]
    pub fn id(&self) -> wgpu::Id {
        self._buffer.global_id()
    }
}

pub(crate) struct ComputeContext<'a> {
    bindgroup: wgpu::BindGroup,
    module: wgpu::ShaderModule,
    pass: wgpu::ComputePass<'a>,
    pipeline: wgpu::ComputePipeline,
}

impl ComputeContext<'_> {
    // Wrapper around a compute shader and its components, shader language module should be ready by this point
    // pub fn pack(encoder: wgpu::CommandEncoder, pipeline: wgpu::ComputePipeline) -> Result<Self, wgpu::Error> {}
    pub fn pack(bindgroup: wgpu::BindGroup, _module: Cow<'_, str>) -> Result<Self, wgpu::Error> {
        let handler = unsafe { Handler::request()?.as_mut().unwrap() };

        let (module, pipeline) = handler
            .load_module(_module)
            .expect("Processed corrupt module.");

        let pass = handler.start_pass()?;

        let context = Self {
            bindgroup,
            module,
            pass,
            pipeline,
        };

        Ok(context)
    }
    pub fn run(&self) {}
}
