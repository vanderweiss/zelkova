use bytemuck::{self, NoUninit};

use super::handler::Handler;

mod _sealed {
    pub trait Sealed {}
}

pub trait Component: _sealed::Sealed + NoUninit {}

macro_rules! impl_component {
    ($($ty:ident)*) => {$(
        impl Component for $ty {}
        impl _sealed::Sealed for $ty {}
    )*}
}

// Valid types to operate on buffers
impl_component! {
    u16 u32 u64
    i16 i32 i64
    f32 f64
}

/// Abstraction layer for wgpu::Buffer.
pub(crate) struct Buffer {
    pub binding: u32,

    #[doc(hidden)]
    _buffer: wgpu::Buffer,
}

impl Buffer {
    pub fn bind<C: Component>(content: &[C], binding: u32) -> Result<Self, wgpu::Error> {
        let _buffer = unsafe {
            Handler::request()?
                .as_ref()
                .unwrap()
                .alloc_buffer_init(bytemuck::cast_slice::<C, u8>(content))?
        };

        let entry = Self { _buffer, binding };

        Ok(entry)
    }

    pub fn group(&self) -> Result<wgpu::BindGroupEntry, wgpu::Error> {
        let _group = wgpu::BindGroupEntry {
            binding: self.binding,
            resource: self._buffer.as_entire_binding(),
        };

        Ok(_group)
    }

    pub fn layout(&self) -> Result<wgpu::BindGroupLayoutEntry, wgpu::Error> {
        let _layout = wgpu::BindGroupLayoutEntry {
            binding: self.binding,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };

        Ok(_layout)
    }

    #[inline]
    pub fn free(&self) {
        drop(self._buffer.slice(..).get_mapped_range());
    }

    #[inline]
    pub fn id(&self) -> wgpu::Id {
        self._buffer.global_id()
    }

    pub fn permissions(&self) -> String {
        let matches = [
            "map_read",
            "map_write",
            "copy_src",
            "copy_dst",
            "index",
            "vertex",
            "uniform",
            "storage",
            "indirect",
            "query_resolve",
        ];

        let valid: Vec<&str> = matches
            .iter()
            .enumerate()
            .filter(|(i, _)| self._buffer.usage().bits() == 1 << i)
            .map(|(_, v)| *v)
            .collect();

        let mut format = String::new();

        for usage in valid.iter() {
            format.push_str(usage);
            if usage != valid.last().unwrap() {
                format.push_str(", ");
            }
        }

        format
    }
}
