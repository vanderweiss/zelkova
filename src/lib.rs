use std::collections::HashMap;
use std::mem;

use bytemuck::NoUninit;
use thiserror::Error;
use tokio::sync::oneshot;

use wgpu::{
    self, include_wgsl, util::DeviceExt, BufferUsages, DeviceType
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unable to find a GPU! Make sure you have installed required drivers!")]
    GpuNotFound,
}

mod element {
    /// Prevent others from implementing Element for their own types.
    pub trait Sealed {}
}

/// Valid element types to operate on.
pub trait Element: element::Sealed + NoUninit {}

macro_rules! impl_element {
    ($($ident:ident)*) => {$(
        impl Element for $ident {}
        impl element::Sealed for $ident {}
    )*}
}

impl_element! {
    f32 f64
    i8 i16 i32 i64 isize
    u8 u16 u32 u64 usize
}

pub struct Handler {
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl Handler {
    /// Initialize the device.
    pub async fn new() -> Result<Self, Error> {
        let instance = wgpu::Instance::default();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .ok_or(Error::GpuNotFound)?;

        let adapter_info = adapter.get_info();

        tracing::info!("{adapter_info:?}");

        if matches!(adapter_info.device_type, DeviceType::Cpu) {
            tracing::warn!("Adapter is llvmpipe");
        }

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some(env!("CARGO_PKG_NAME")),
                    ..Default::default()
                },
                None,
            )
            .await
            .unwrap();

        Ok(Self { device, queue })
    }

    /// Create a GPU buffer from a slice of `T`.
    fn create_buffer<T: Element>(
        &self,
        label: &'static str,
        buffer: &[T],
        usage: BufferUsages,
    ) -> wgpu::Buffer {
        self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(label),
                contents: bytemuck::cast_slice::<T, u8>(buffer),
                usage,
            })
    }

    /// Create an uninitialized GPU buffer of `T`s.
    fn create_uninit_buffer<T: Element>(
        &self,
        label: &'static str,
        len: usize,
        usage: BufferUsages,
    ) -> wgpu::Buffer {
        self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size: (len * mem::size_of::<T>()) as _,
            usage,
            mapped_at_creation: false,
        })
    }

    /// Add lhs and rhs, returning the result.
    pub async fn add(&self, lhs: &[f32], rhs: &[f32]) -> Vec<f32> {
        assert_eq!(lhs, rhs, "input slices must be same length");

        let module = self
            .device
            .create_shader_module(include_wgsl!("shader.wgsl"));

        let lhs_buffer = self.create_buffer(
            "lhs",
            lhs,
            BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
        );
        let rhs_buffer = self.create_buffer(
            "rhs",
            rhs,
            BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
        );
        let output_buffer = self.create_uninit_buffer::<f32>(
            "output",
            rhs.len(),
            BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        );

        let pipeline = self
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: None,
                layout: None,
                module: &module,
                entry_point: "add",
            });

        let bind_group_layout = pipeline.get_bind_group_layout(0);
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: lhs_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: rhs_buffer.as_entire_binding(),
                },
            ],
        });

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });

            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.set_bind_group(1, &bind_group, &[]);
            pass.insert_debug_marker("compute add");
            pass.dispatch_workgroups(lhs.len() as u32, 1, 1);
        }

        encoder.copy_buffer_to_buffer(
            &lhs_buffer,
            0,
            &output_buffer,
            0,
            (lhs.len() * mem::size_of::<f32>()) as _,
        );

        self.queue.submit(Some(encoder.finish()));

        let output_slice = output_buffer.slice(..);
        let (tx, rx) = oneshot::channel();

        output_slice.map_async(wgpu::MapMode::Read, move |result| tx.send(result).unwrap());

        self.device.poll(wgpu::Maintain::Wait);

        if let Ok(Ok(())) = rx.await {
            let data = output_slice.get_mapped_range();
            let result: Vec<f32> = bytemuck::cast_slice(&data).to_vec();

            result
        } else {
            panic!("no computed")
        }
    }
}

struct SharedBuffer {
    binding: i32,
    group: i32,
}

struct Context<'a, T: Element> {
    handler: &'a Handler,
    shader: &'static str,
    lhs: &'a [T], 
    rhs: &'a [T],
}

impl<'a, T: Element> Context<'a, T> {
    
    fn build_pipeline(&self, module: wgpu::ShaderModule) -> Result<wgpu::ComputePipeline, Error> {

        let pipeline = self
            .handler
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: None, layout: None,
                module: &module, 
                entry_point: self.shader,
            });

        Ok(pipeline)
    }
}








