use wgpu;

use pollster;

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

    pub fn try_encoder(&self) -> Result<wgpu::CommandEncoder, wgpu::Error> {
        let encoder = self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        Ok(encoder)
    }
}

pub struct ComputeContext {
    bindgroup: wgpu::BindGroup,
    pass: wgpu::ComputePass,
    pipeline: wgpu::ComputePipeline,
}

impl ComputeContext {
    // Wrapper around a comput shader and its components
    pub fn create(encoder: wgpu::CommandEncoder, module: wgpu::ShaderModule) -> Result<Self, wgpu::Error> { 
        
    }

    pub fn run() {}
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

