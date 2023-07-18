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
pub(crate) enum Memory {
    #[default]
    Static,
    Runtime,
}

#[derive(Clone, Default)]
pub(crate) enum State {
    #[default]
    Binded,
    Prepared,
}

#[derive(Builder)]
pub(crate) struct Properties {
    pub alias: &'static str,
    pub binding: u32,
    pub count: usize,

    #[builder(default)]
    pub group: u32,

    #[builder(default)]
    pub memory: Memory,

    #[builder(default)]
    pub state: State,
}

impl Properties {
    pub fn construct(alias: &'static str, count: usize, memory: Option<Memory>) -> Self {
        static Tracker: AtomicU32 = AtomicU32::new(0);
        let binding = Tracker.fetch_add(1, Ordering::SeqCst);

        let mut builder = PropertiesBuilder::default();
        builder
            .alias(alias)
            .binding(binding)
            .count(count)
            .memory(memory.unwrap_or_default())
            .build()
            .unwrap()
    }

    pub fn prepare(&mut self) {
        self.state = State::Prepared;
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
    pub fn bind_st<C: Component, const N: usize>(_src: &[C; N]) -> Result<Self, wgpu::Error> {
        let props = Properties::construct(any::type_name::<C>(), N, None);

        let bundle = Self {
            buffer: BufferHolder::new(),
            props,
        };

        dbg!(bundle.tag());

        Ok(bundle)
    }

    pub fn bind_rt<C: Component>(_src: &[C]) -> Result<Self, wgpu::Error> {
        let props = Properties::construct(any::type_name::<C>(), _src.len(), Some(Memory::Runtime));

        let bundle = Self {
            buffer: BufferHolder::new(),
            props,
        };

        dbg!(bundle.tag());

        Ok(bundle)
    }
}

use crate::codegen::Element;
