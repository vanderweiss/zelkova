use {
    bytemuck::Pod,
    std::{
        borrow::Cow,
        collections::HashMap,
        fmt::Write,
        num::{NonZeroU32, NonZeroU64},
    },
    wgpu::{
        util::{BufferInitDescriptor, DeviceExt},
        BufferUsages, ShaderSource,
    },
};

/// A device.
#[derive(Debug)]
pub struct Device {
    bindings: HashMap<Box<str>, wgpu::Buffer>,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl Device {
    /// Declare a shader binding initialized with data.
    pub fn bind<T: Pod>(&mut self, label: &str, data: &mut T) {
        const USAGE: BufferUsages = BufferUsages::COPY_DST
            .union(BufferUsages::COPY_SRC)
            .union(BufferUsages::STORAGE);

        let buffer = self.device.create_buffer_init(&BufferInitDescriptor {
            label: Some(label),
            contents: bytemuck::bytes_of_mut(data),
            usage: USAGE,
        });

        self.bindings.insert(Box::from(label), buffer);
    }

    /// Declare a shader binding containing zeros, `len` is in bytes.
    pub fn bind_zeroed(&mut self, label: &str, len: usize) {
        const USAGE: BufferUsages = BufferUsages::COPY_DST.union(BufferUsages::MAP_READ);

        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size: len as wgpu::BufferAddress,
            usage: USAGE,
            mapped_at_creation: false,
        });

        self.bindings.insert(Box::from(label), buffer);
    }

    pub fn compute(&mut self, body: &str, id: [u32; 3]) {
        let mut shader = String::new();

        let mut entries_layout = Vec::new();
        let mut entries_binding = Vec::new();

        // generate both shader bindings and bind group entries
        for (index, (label, buffer)) in self.bindings.iter().enumerate() {
            writeln!(
                &mut shader,
                "@group(0) @binding({index}) var<storage, read_write> {label}: array<f32>;"
            )
            .unwrap();

            entries_layout.push(wgpu::BindGroupLayoutEntry {
                binding: index as u32,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU64::new(32),
                },
                count: NonZeroU32::new(32),
            });

            entries_binding.push(wgpu::BindGroupEntry {
                binding: index as u32,
                resource: buffer.as_entire_binding(),
            });
        }

        // generate the main function
        writeln!(
            &mut shader,
            "@compute @workgroup_size(1) fn main(@builtin(global_invocation_id) index: vec3<u32>) {{"
        )
        .unwrap();

        // add the body
        shader.push_str(body);

        writeln!(&mut shader, "}}").unwrap();

        println!("{shader}");

        let source = ShaderSource::Wgsl(Cow::Owned(shader));

        // create a shader module
        let module = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("zelkova.wgsl"),
                source,
            });

        // create a pipeline
        let pipeline = self
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: None,
                layout: None,
                module: &module,
                entry_point: "main",
            });

        let bind_group_layout =
            self.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &entries_layout,
                });

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &entries_binding,
        });

        let mut encoder = self.device.create_command_encoder(&Default::default());

        {
            let mut pass = encoder.begin_compute_pass(&Default::default());

            pass.set_pipeline(&pipeline);

            for binding in entries_binding.iter() {
                pass.set_bind_group(binding.binding, &bind_group, &[]);
            }

            let [x, y, z] = id;

            pass.dispatch_workgroups(x, y, z);
        }

        self.queue.submit(Some(encoder.finish()));
    }
}

impl Default for Device {
    fn default() -> Self {
        pollster::block_on(async move {
            let instance = wgpu::Instance::default();

            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions::default())
                .await
                .unwrap();

            let (device, queue) = adapter
                .request_device(&wgpu::DeviceDescriptor::default(), None)
                .await
                .unwrap();

            Device {
                bindings: HashMap::new(),
                device,
                queue,
            }
        })
    }
}
