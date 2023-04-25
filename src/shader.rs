// Main abstraction layer between wgpu and the low level user API

use {
    std::{
        borrow::Cow,
        default::Default,  
    },
    wgpu::{
        self,
    },
    pollster::{
        self,
    },
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

            let encoder = device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

            let handler = Self { 
                adapter,
                device,
                encoder,
                queue,
            };

            Ok(handler)
        }) 
    }

    pub fn load_module(&self, module: Cow<'_, str>) -> Result<(wgpu::ShaderModule, wgpu::ComputePipeline), wgpu::Error> { 
        let module = self.device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(module),
            });

        let pipeline = self.device
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
    binding: wgpu::BindGroupEntry<'a>,
    layout: wgpu::BindGroupLayoutEntry,
    buffer: wgpu::Buffer,
}

impl BufferEntry<'_> {
    // Returns Self
    /*pub fn bind<T, const N: usize>(container: &[T; N]) -> Self { 
    }*/

    pub fn free() {}
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
