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

    #[inline]
    pub fn bits(&self) -> u32 {
        self._buffer.usage().bits()
    }

    #[inline]
    pub fn contains(&self, bits: u32) -> bool {
        let flags = wgpu::BufferUsages::from_bits(bits).unwrap();
        self._buffer.usage().contains(flags)
    }

    #[inline]
    pub fn free(&self) {
        drop(self._buffer.slice(..).get_mapped_range());
    }

    #[inline]
    pub fn id(&self) -> wgpu::Id {
        self._buffer.global_id()
    }

    #[inline]
    pub fn is_storage(&self) -> bool {
        self._buffer.usage().contains(wgpu::BufferUsages::STORAGE)
    }

    #[inline]
    pub fn is_uniform(&self) -> bool {
        self._buffer.usage().contains(wgpu::BufferUsages::UNIFORM)
    }

    #[inline]
    pub fn resource(&self) -> wgpu::BindingResource {
        self._buffer.as_entire_binding()
    }
}

pub(crate) struct BufferMeta<'b> {
    pub buffer: &'b Buffer,

    #[doc(hidden)]
    _group: wgpu::BindGroupEntry<'b>,
    #[doc(hidden)]
    _layout: wgpu::BindGroupLayoutEntry,
}

impl<'b> BufferMeta<'b> {
    pub fn from_buffer(binding: u32, buffer: &'b Buffer) -> Result<Self, wgpu::Error> {
        let _group = wgpu::BindGroupEntry {
            binding,
            resource: buffer.resource(),
        };

        let ty = {
            if buffer.is_storage() {
                wgpu::BufferBindingType::Storage { read_only: false }
            } else if buffer.is_uniform() {
                wgpu::BufferBindingType::Uniform
            } else {
                panic!()
            }
        };

        let visibility = wgpu::ShaderStages::COMPUTE;

        let _layout = wgpu::BindGroupLayoutEntry {
            binding,
            visibility,
            ty: wgpu::BindingType::Buffer {
                ty,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };

        let meta = Self {
            buffer,
            _group,
            _layout,
        };

        Ok(meta)
    }
}
