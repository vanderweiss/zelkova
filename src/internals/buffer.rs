use {bytemuck, std::mem, wgpu};

use crate::types::Component;

use super::Handler;

/// Identifiers for `Buffer`s
#[derive(Clone, Copy, Default)]
pub(crate) enum BufferType {
    #[default]
    Init,
    Map,
}

/// Abstraction layer for wgpu::Buffer.
pub(crate) struct Buffer {
    pub ty: BufferType,

    #[doc(hidden)]
    _buffer: wgpu::Buffer,
}

impl Buffer {
    pub fn bind<T>(
        handler: Handler,
        ty: BufferType,
        _content: Option<&[T]>,
        _size: Option<u64>,
    ) -> Result<Self, wgpu::Error>
    where
        T: Component,
    {
        let _buffer = {
            match ty {
                BufferType::Init => handler.alloc_buffer_init({
                    if let Some(content) = _content {
                        bytemuck::cast_slice::<T, u8>(content)
                    } else {
                        panic!()
                    }
                })?,
                BufferType::Map => handler.alloc_buffer_map({
                    if let Some(size) = _size {
                        size
                    } else {
                        mem::size_of::<T> as u64
                    }
                })?,
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

    /// Generic typing to be made clear.
    /*
    #[inline]
    pub fn id(&self) -> wgpu::Id<wgpu_> {
        self._buffer.global_id()
    }
    */

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
