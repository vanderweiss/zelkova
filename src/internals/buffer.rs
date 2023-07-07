use {
    bytemuck::{self, NoUninit},
    std::mem,
    wgpu,
};

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

/// Identifiers for buffer usage
pub(crate) enum BufferType {
    Staging,
    Factor,
}

/// Abstraction layer for wgpu::Buffer.
pub(crate) struct Buffer {
    pub ty: BufferType,

    #[doc(hidden)]
    _buffer: wgpu::Buffer,
}

impl Buffer {
    pub fn bind<C: Component>(ty: BufferType, content: Option<&[C]>) -> Result<Self, wgpu::Error> {
        let _buffer = unsafe {
            let handler = Handler::request()?.as_ref().unwrap();
            match ty {
                BufferType::Factor => handler.alloc_buffer_factor(
                    bytemuck::cast_slice::<C, u8>(content.unwrap_unchecked()),
                )?,
                BufferType::Staging => handler.alloc_buffer_staging(mem::size_of::<C>())?,
            }
        };

        let entry = Self { _buffer, ty };

        Ok(entry)
    }

    pub fn group(&self, binding: u32) -> Result<wgpu::BindGroupEntry, wgpu::Error> {
        let _group = wgpu::BindGroupEntry {
            binding,
            resource: self._buffer.as_entire_binding(),
        };

        Ok(_group)
    }

    pub fn layout(&self, binding: u32) -> Result<wgpu::BindGroupLayoutEntry, wgpu::Error> {
        let _layout = wgpu::BindGroupLayoutEntry {
            binding,
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
    pub fn bits(&self) -> u32 {
        if self._buffer.usage().contains_invalid_bits() {
            panic!()
        } else {
            self._buffer.usage().bits()
        }
    }

    #[inline]
    pub fn free(&self) {
        drop(self._buffer.slice(..).get_mapped_range());
    }

    #[inline]
    pub fn id(&self) -> wgpu::Id {
        self._buffer.global_id()
    }
}
