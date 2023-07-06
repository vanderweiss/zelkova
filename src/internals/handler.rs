use {
    pollster,
    std::{
        borrow::Cow,
        ptr,
        sync::{LazyLock, Mutex},
    },
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
