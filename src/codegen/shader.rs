// Main abstraction layer between wgpu and the low level user API

use {
    bytemuck::{self, NoUninit},
    pollster,
    std::{borrow::Cow, string::ToString, sync::LazyLock},
    wgpu::{self, util::DeviceExt},
};

mod _sealed {
    pub trait Sealed {}
}

pub(crate) trait Component: _sealed::Sealed + NoUninit {}

macro_rules! impl_component {
    ($($ident:ident)*) => {$(
        impl Component for $ident {}
        impl _sealed::Sealed for $ident {}
    )*}
}

// Valid types to operate on buffers
impl_component! {
    u16 u32 u64
    i16 i32 i64
    f32 f64
}

pub(crate) struct Owned;

impl Owned {
    #[inline]
    pub fn from(resource: impl ToString) -> Option<&'static str> {
        Some(format!("Zelkova owned {}.", resource.to_string()).as_str())
    }
}

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

    #[allow(non_upper_case_globals)]
    #[must_use]
    pub fn request() -> Result<&'static Self, wgpu::Error> {
        static _handler: LazyLock<Handler> =
            LazyLock::new(|| Handler::_request().expect("Failed to connect to GPU"));

        Ok(&_handler)
    }

    #[inline]
    pub fn start_pass(&mut self) -> Result<wgpu::ComputePass, wgpu::Error> {
        let pass = self
            .encoder
            .begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Owned::from("compute pass"),
            });

        Ok(pass)
    }

    pub fn load_module(
        &self,
        module: Cow<'_, str>,
    ) -> Result<(wgpu::ShaderModule, wgpu::ComputePipeline), wgpu::Error> {
        let module = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Owned::from("module"),
                source: wgpu::ShaderSource::Wgsl(module),
            });

        let pipeline = self
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Owned::from("pipeline"),
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
                label: Owned::from("buffer"),
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
            label: Owned::from("bind group"),
            layout: &self
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Owned::from("bind group layout"),
                    entries: meta,
                }),
            entries,
        });

        Ok(group)
    }
}

pub(crate) struct BufferEntry {
    buffer: wgpu::Buffer,
    binding: u32,
}

impl BufferEntry {
    pub fn bind<C: Component>(content: &[C], binding: u32) -> Result<Self, wgpu::Error> {
        let buffer =
            Handler::request()?.alloc_buffer_init(bytemuck::cast_slice::<C, u8>(content))?;

        let entry = Self { buffer, binding };

        Ok(entry)
    }

    pub fn group(&self) -> Result<wgpu::BindGroupEntry, wgpu::Error> {
        let _group = wgpu::BindGroupEntry {
            binding: self.binding,
            resource: self.buffer.as_entire_binding(),
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
    pub fn id(&self) -> wgpu::Id {
        self.buffer.global_id()
    }

    #[inline]
    pub fn free(&self) {
        drop(self.buffer.slice(..).get_mapped_range());
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
        let handler = Handler::request()?;

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
