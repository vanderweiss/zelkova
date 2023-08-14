use {
    std::{
        any,
        marker::Sized,
        mem::MaybeUninit,
        ops::{Deref, DerefMut},
        sync::atomic::{AtomicU32, Ordering},
    },
    wgpu,
};

use crate::internals::{Buffer, Component};

#[derive(Clone, Default)]
pub(crate) enum Future {
    #[default]
    Pending,
    Done,
}

#[derive(Clone)]
pub(crate) enum Group {
    Base,
    Custom(u32),
}

impl Default for Group {
    fn default() -> Group {
        Group::Base
    }
}

#[derive(Clone, Default)]
pub(crate) enum Memory {
    #[default]
    Static,
    Runtime,
}

#[derive(Clone, Default)]
pub(crate) enum Type {
    #[default]
    Result,
    Future,
}

pub(crate) struct GenOpts {
    pub alias: &'static str,
    pub binding: u32,
    pub count: usize,
}

#[derive(Default)]
pub(crate) struct SpecOpts {
    pub future: Future,
    pub group: Group,
    pub memory: Memory,
    pub ty: Type,
}

pub(crate) struct Properties {
    pub gen: GenOpts,
    pub spec: SpecOpts,
}

impl Properties {
    pub fn construct(alias: &'static str, count: usize) -> Self {
        static Tracker: AtomicU32 = AtomicU32::new(0);

        let binding = Tracker.fetch_add(1, Ordering::SeqCst);

        let props = Self {
            gen: GenOpts {
                alias,
                binding,
                count,
            },
            spec: SpecOpts::default(),
        };

        props
    }

    #[inline]
    pub fn alias(&self) -> &'static {
        self.gen.alias
    }

    #[inline]
    pub fn binding(&self) -> u32 {
        self.gen.binding
    }

    #[inline]
    pub fn count(&self) -> usize {
        self.gen.count
    }
}

pub(crate) struct BufferHolder {
    buffer: MaybeUninit<Buffer>,
    init: bool,
}

impl BufferHolder {
    pub fn new() -> Self {
        Self {
            buffer: MaybeUninit::uninit(),
            init: false,
        }
    }

    pub fn initialize(&mut self, _buffer: Buffer) {
        self.init = true;
        self.buffer.write(_buffer);
    }
}

impl Deref for BufferHolder {
    type Target = Buffer;

    fn deref(&self) -> &Self::Target {
        unsafe { self.buffer.assume_init_ref() }
    }
}

impl DerefMut for BufferHolder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.buffer.assume_init_mut() }
    }
}

/// Interface on top of the toolkit's wrapper for buffers, used for shader generation and extends
/// to other api-related structures.
pub(crate) struct Bundle {
    pub buffer: BufferHolder,
    pub props: Properties,
}

impl Bundle {
    pub fn bind_st<C: Component>(count: usize) -> Result<Self, wgpu::Error> {
        let props = Properties::construct(any::type_name::<C>(), count, None);

        let bundle = Self {
            buffer: BufferHolder::new(),
            props,
        };

        dbg!(bundle.tag());

        Ok(bundle)
    }

    pub fn bind_rt<C: Component>(count: usize) -> Result<Self, wgpu::Error> {
        let props = Properties::construct(any::type_name::<C>(), count, Some(Memory::Runtime));

        let bundle = Self {
            buffer: BufferHolder::new(),
            props,
        };

        dbg!(bundle.tag());

        Ok(bundle)
    }
}

use crate::codegen::Element;
