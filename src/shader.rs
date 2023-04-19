use {
    wgpu::{
        self,
    },
    pollster::{
        self,
    },
};

// Main abstraction layer for wgpu internals
pub struct Handler {
    device: wgpu::Device,
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

            let handler = Self { device, queue };

            Ok(handler)
        }) 
    }

    pub fn retrieve_encoder(&self) -> Result<wgpu::CommandEncoder, wgpu::Error> {
        let encoder = self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        Ok(encoder)
    }

    pub fn construct_pipeline(&self, module: &wgpu::ShaderModule, entry_point: &'static str) -> Result<wgpu::ComputePipeline, wgpu::Error> {
        let pipeline = self.device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: None,
                layout: None,
                module, 
                entry_point,
            });
        Ok(pipeline)
    }
}

pub struct BufferEntry {
    binding: wgpu::BindGroupEntry,
    layout: wgpu::BindGroupLayoutEntry,
    buffer: wgpu::Buffer,
}

impl BufferEntry {
    pub fn process() -> Self {}
}

pub struct ComputeContext {
    bindgroup: wgpu::BindGroup,
    pass: wgpu::ComputePass,
    pipeline: wgpu::ComputePipeline,
}

impl ComputeContext {
    // Wrapper around a comput shader and its components
    pub fn create(encoder: wgpu::CommandEncoder, pipeline: wgpu::ComputePipeline) -> Result<Self, wgpu::Error> {        
       let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
    }

    pub fn run(&self) {}
}

pub trait ShaderHelper {
    fn base(device: &wgpu::Device) -> Result<wgpu::ShaderModule, wgpu::Error>;
}

impl ShaderHelper for wgpu::ShaderModule {
    // Utils for modules
    fn base(device: &wgpu::Device) -> Result<wgpu::ShaderModule, wgpu::Error> {
        let module = device
            .create_shader_module(wgpu::include_wgsl!("zelkova.wsgl"));
        Ok(module) 
    } 
}
