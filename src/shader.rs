// Main abstraction layer between wgpu and the low level user API

use {
    pollster,
    std::{
        borrow::Cow,
        default::Default,
        mem,
        num::{NonZeroU32, NonZeroU64},
        sync::LazyLock,
    },
    wgpu::{self, util::DeviceExt},
};

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

    pub fn alloc_buffer_init(&self, contents: &[u8]) -> Result<wgpu::Buffer, wgpu::Error> {
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

pub struct BufferEntry<'a> {
    binding: wgpu::BindGroupEntry<'a>,
    layout: wgpu::BindGroupLayoutEntry,
}

impl BufferEntry<'_> {
    pub fn bind<T, const N: usize>(content: &[T; N], index: u32) -> Result<Self, wgpu::Error> {
        let buffer = Handler::request()?
            .alloc_buffer_init(unsafe { mem::transmute::<&[T], &[u8]>(content) })?;

        let entry = Self {
            binding: wgpu::BindGroupEntry {
                binding: index,
                resource: buffer.as_entire_binding().clone(),
            },
            layout: wgpu::BindGroupLayoutEntry {
                binding: index,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU64::new(buffer.size()),
                },
                count: NonZeroU32::new(1),
            },
        };

        Ok(entry)
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
