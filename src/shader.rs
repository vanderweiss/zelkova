// Main abstraction layer between wgpu and the low level user API

use {
    pollster::{self},
    std::{borrow::Cow, default::Default, num::{NonZeroU32, NonZeroU64}},
    wgpu::{self},
};

// Interface to handle wgpu internals
pub struct Handler {
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    encoder: wgpu::CommandEncoder,
    queue: wgpu::Queue,
}

impl Handler {
    pub fn request() -> Result<Self, wgpu::Error> {
        pollster::block_on(async move {
            let adapter = wgpu::Instance::default()
                .request_adapter(&wgpu::RequestAdapterOptions::default())
                .await
                .unwrap();

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
}

pub struct BufferEntry<'a> {
    buffer: wgpu::Buffer,
    binding: wgpu::BindGroupEntry<'a>,
    layout: wgpu::BindGroupLayoutEntry,
}

impl BufferEntry<'_> {
    pub fn bind<T, const N: usize>(content: &[T; N], index: u32) -> Self {
        BufferEntry {
            buffer: None, // placeholder
            binding: wgpu::BindGroupEntry {
                binding: index,
                resource: wgpu::BindingResource::Buffer(), // placeholder
            },
            layout: wgpu::BindGroupLayoutEntry {
                binding: index,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType,
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU64::new(1),
                },
                count: NonZeroU32::new(1), 
            },
        }
    }

    pub fn free(&self) {}
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
