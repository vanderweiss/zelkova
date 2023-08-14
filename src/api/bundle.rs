use {
    std::{
        any,
        mem::MaybeUninit,
        ops::{Deref, DerefMut},
        sync::atomic::{AtomicU32, Ordering},
    },
    wgpu,
};

use crate::internals::{Buffer, Component};

#[derive(Clone, Copy)]
pub(crate) enum Group {
    Base,
    Custom(u32),
}

impl Default for Group {
    fn default() -> Group {
        Group::Base
    }
}

#[derive(Clone, Copy, Default)]
pub(crate) enum Memory {
    #[default]
    Static,
    Runtime,
}

#[derive(Clone, Copy, Default)]
pub(crate) enum State {
    #[default]
    Pending,
    Done,
}

#[derive(Clone, Copy)]
pub(crate) enum Type {
    Result,
    Future(State),
}

impl Default for Type {
    fn default() -> Self {
        Type::Result
    }
}

pub(crate) struct GenOpts {
    pub alias: &'static str,
    pub binding: u32,
    pub length: usize,
}

#[derive(Default)]
pub(crate) struct SpecOpts {
    pub group: Group,
    pub memory: Memory,
    pub ty: Type,
}

pub(crate) struct Properties {
    pub gen: GenOpts,
    pub spec: SpecOpts,
}

impl Properties {
    pub fn construct(alias: &'static str, length: usize) -> Self {
        static Tracker: AtomicU32 = AtomicU32::new(0);

        let binding = Tracker.fetch_add(1, Ordering::SeqCst);

        let props = Self {
            gen: GenOpts {
                alias,
                binding,
                length,
            },
            spec: SpecOpts::default(),
        };

        props
    }

    #[inline]
    pub fn alias(&self) -> &'static str {
        self.gen.alias
    }

    #[inline]
    pub fn binding(&self) -> u32 {
        self.gen.binding
    }

    #[inline]
    pub fn length(&self) -> usize {
        self.gen.length
    }

    #[inline]
    pub fn group(&self) -> u32 {
        match self.spec.group {
            Group::Base => 0,
            Group::Custom(group) => group,
        }
    }

    #[inline]
    pub fn ready(&mut self) -> bool {
        match self.spec.ty {
            Type::Result => true,
            Type::Future(state) => match state {
                State::Pending => false,
                State::Done => {
                    self.spec.ty = Type::Result;
                    true
                }
            },
        }
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
    pub fn bind_st<C: Component>(length: usize) -> Result<Self, wgpu::Error> {
        let props = Properties::construct(any::type_name::<C>(), length);

        let bundle = Self {
            buffer: BufferHolder::new(),
            props,
        };

        Ok(bundle)
    }

    pub fn bind_rt<C: Component>(length: usize) -> Result<Self, wgpu::Error> {
        let props = Properties::construct(any::type_name::<C>(), length);

        let bundle = Self {
            buffer: BufferHolder::new(),
            props,
        };

        Ok(bundle)
    }
}
