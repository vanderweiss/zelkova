use {
    std::{
        any,
        mem::MaybeUninit,
        ops::{Deref, DerefMut},
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

    #[builder(default)]
    pub group: u32,

    #[builder(default)]
    pub memory: Memory,

    #[builder(default)]
    pub state: State,
}

impl Properties {
    pub fn construct(alias: &'static str, binding: u32, memory: Option<Memory>) -> Self {
        let mut builder = PropertiesBuilder::default();
        builder
            .alias(alias)
            .binding(binding)
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

    pub fn init(&mut self, _buffer: Buffer) {
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
    pub fn bind<C: Component>(_src: &[C], binding: u32) -> Result<Self, wgpu::Error> {
        let buffer = BufferHolder::new();
        let props = Properties::construct(any::type_name::<C>(), binding, None);

        let bundle = Self { buffer, props };

        Ok(bundle)
    }
}

use crate::codegen::Element;
