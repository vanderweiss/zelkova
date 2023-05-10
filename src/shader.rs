// Main abstraction layer between wgpu and the low level user API

use {
    bytemuck::{self, NoUninit},
    pollster,
    std::{
        borrow::Cow,
        default::Default,
        num::{NonZeroU32, NonZeroU64},
        sync::LazyLock,
    },
    wgpu::{self, util::DeviceExt},
};

mod _sealed {
    pub trait Sealed {}
}

pub trait Component: _sealed::Sealed + NoUninit {}

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

pub type _Bty = u8; // Buffer conversion - from primitive slices
pub type _Tty = u32; // Tracker type - layout util

// Core interface to handle wgpu internals
pub struct Handler {
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    encoder: wgpu::CommandEncoder,
    queue: wgpu::Queue,
}

impl Handler {
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

    pub fn request() -> Result<&'static Self, wgpu::Error> {
        static _handler: LazyLock<Handler> =
            LazyLock::new(|| Handler::_request().expect("Failed to connect to GPU"));

        Ok(&_handler)
    }

    pub fn load_module(
        &self,
        module: Cow<'_, str>,
    ) -> Result<(wgpu::ShaderModule, wgpu::ComputePipeline), wgpu::Error> {
        let module = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(module),
            });

        let pipeline = self
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: None,
                layout: None,
                module: &module,
                entry_point: "main",
            });

        Ok((module, pipeline))
    }

    pub fn alloc_buffer_init(&self, contents: &[_Bty]) -> Result<wgpu::Buffer, wgpu::Error> {
        let buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents,
                usage: wgpu::BufferUsages::MAP_READ,
            });

        Ok(buffer)
    }

    pub fn alloc_uninit_buffer(&self) {}
}

pub struct BufferEntry {
    buffer: wgpu::Buffer,
    layout: wgpu::BindGroupLayoutEntry,
    index: _Tty,
}

impl BufferEntry {
    pub fn bind<C: Component, const N: usize>(
        content: &[C],
        index: _Tty,
    ) -> Result<Self, wgpu::Error> {
        let buffer =
            Handler::request()?.alloc_buffer_init(bytemuck::cast_slice::<C, _Bty>(content))?;

        let min_binding_size = NonZeroU64::new(buffer.size());

        let entry = Self {
            buffer,
            layout: wgpu::BindGroupLayoutEntry {
                binding: index,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size,
                },
                count: NonZeroU32::new(N as u32),
            },
            index,
        };

        Ok(entry)
    }

    pub fn in_group(&self) -> Result<wgpu::BindGroupEntry, wgpu::Error> {
        let location = wgpu::BindGroupEntry {
            binding: self.index,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &self.buffer,
                offset: self.buffer.size(),
                size: None, // entire buffer
            }),
        };

        Ok(location)
    }

    pub fn free(&self) {
        // Gets dropped later as a means of confirmation
        //drop(self.buffer.slice(..).get_mapped_range());
    }
}

pub struct ComputeContext<'a> {
    bindgroup: wgpu::BindGroup,
    pass: wgpu::ComputePass<'a>,
    pipeline: wgpu::ComputePipeline,
}

impl ComputeContext<'_> {
    // Wrapper around a compute shader and its components
    // pub fn pack(encoder: wgpu::CommandEncoder, pipeline: wgpu::ComputePipeline) -> Result<Self, wgpu::Error> {}

    pub fn run(&self) {}
}
