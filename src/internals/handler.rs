use {
    pollster,
    std::borrow::Cow,
    wgpu::{self, util::DeviceExt},
};

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

    pub fn start_pass(&mut self) -> Result<wgpu::ComputePass, wgpu::Error> {
        let pass = self
            .encoder
            .begin_compute_pass(&wgpu::ComputePassDescriptor { label });

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

    pub fn alloc_buffer_factor(&self, contents: &[u8]) -> Result<wgpu::Buffer, wgpu::Error> {
        let usage = wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST;

        let buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label,
                contents,
                usage,
            });

        Ok(buffer)
    }

    pub fn alloc_buffer_staging(&self, size: usize) -> Result<wgpu::Buffer, wgpu::Error> {
        let size = size as u64;
        let usage = wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST;

        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label,
            size,
            usage,
            mapped_at_creation: false,
        });

        Ok(buffer)
    }
}
