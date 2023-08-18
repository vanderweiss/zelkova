use {
    std::{
        any,
        marker::PhantomData,
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
pub(crate) enum State {
    #[default]
    Pending,
    Done,
}

pub(crate) struct Operation<C>
where
    C: Component,
{
    op: String,
    state: State,
    ty: PhantomData<C>,
}

impl<C> Operation<C>
where
    C: Component,
{
    pub fn feed(op: String) -> Self {
        Self {
            op,
            state: State::default(),
            ty: PhantomData,
        }
    }

    pub fn resolved(&self) -> bool {
        match self.state {
            State::Pending => false,
            State::Done => true,
        }
    }
}

pub(crate) enum Init<C>
where
    C: Component,
{
    Result,
    Future(Operation<C>),
}

impl<C> Default for Init<C>
where
    C: Component,
{
    fn default() -> Self {
        Init::Result
    }
}

#[derive(Clone, Copy, Default)]
pub(crate) enum Memory {
    #[default]
    Static,
    Runtime,
}

#[derive(Clone, Copy, Default)]
pub(crate) enum Layout {
    #[default]
    Init,
    Future,
    Dyn,
}

pub(crate) struct Properties<C>
where
    C: Component,
{
    pub binding: u32,
    pub group: Group,
    pub init: Init<C>,
    pub length: usize,
    pub memory: Memory,
}

impl<C> Properties<C>
where
    C: Component,
{
    pub fn construct(layout: Layout, length: usize, op: Option<Operation<C>>) -> Self {
        static Tracker: AtomicU32 = AtomicU32::new(0);

        let binding = Tracker.fetch_add(1, Ordering::SeqCst);
        let group = Group::Base;
        let ty: PhantomData<C> = PhantomData;

        let props = match layout {
            Layout::Init => Self {
                binding,
                group,
                init: Init::default(),
                length,
                memory: Memory::default(),
            },
            Layout::Future => Self {
                binding,
                group,
                init: {
                    if let Some(op) = op {
                        Init::Future(op)
                    } else {
                        panic!()
                    }
                },
                length,
                memory: Memory::default(),
            },
            Layout::Dyn => {
                panic!()
            }
        };

        props
    }

    #[inline]
    pub fn group(&self) -> u32 {
        match self.group {
            Group::Base => 0,
            Group::Custom(group) => group,
        }
    }

    #[inline]
    pub fn ready(&self) -> bool {
        match self.init {
            Init::Result => true,
            Init::Future(ref op) => op.resolved(),
        }
    }

    #[inline]
    pub fn typename(&self) -> &'static str {
        any::type_name::<C>()
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

pub(crate) struct Bundle<C>
where
    C: Component,
{
    pub buffer: BufferHolder,
    pub props: Properties<C>,
}

impl<C> Bundle<C>
where
    C: Component,
{
    pub fn bind_init(length: usize) -> Result<Self, wgpu::Error> {
        let props = Properties::construct(Layout::default(), length, None);

        let bundle = Self {
            buffer: BufferHolder::new(),
            props,
        };

        Ok(bundle)
    }

    pub fn bind_future(length: usize, op: Operation<C>) -> Result<Self, wgpu::Error> {
        let props = Properties::construct(Layout::Future, length, Some(op));

        let bundle = Self {
            buffer: BufferHolder::new(),
            props,
        };

        Ok(bundle)
    }
}
