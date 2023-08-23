use {
    std::{
        any,
        fmt::{self, Display, Formatter},
        mem::MaybeUninit,
        ops::{Deref, DerefMut},
        sync::atomic::{AtomicU32, Ordering},
    },
    wgpu,
};

use crate::internals::{Buffer, Component};

use super::Operation;

#[derive(Clone, Copy)]
pub(crate) enum Binding {
    Assigned(u32),
    Hold,
}

impl Binding {
    fn fetch(&self) -> u32 {
        match self {
            Binding::Assigned(binding) => *binding,
            Binding::Hold => panic!(),
        }
    }
}

impl Default for Binding {
    fn default() -> Self {
        static TRACKER: AtomicU32 = AtomicU32::new(0);
        let binding = TRACKER.fetch_add(1, Ordering::SeqCst);
        Binding::Assigned(binding)
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Group {
    Base,
    Custom(u32),
}

impl Group {
    fn fetch(&self) -> u32 {
        match self {
            Group::Base => 0,
            Group::Custom(group) => *group,
        }
    }
}

impl Default for Group {
    fn default() -> Group {
        Group::Base
    }
}

#[derive(Clone)]
pub(crate) enum Length {
    Sized(Vec<u32>),
    Unsized,
}

impl Length {
    fn fetch(&self) -> usize {
        match self {
            Length::Sized(length) => *length,
            Length::Unsized => panic!(),
        }
    }
}

impl PartialEq for Length {
    fn eq(&self, other: &Self) -> bool {
        self.fetch() == other.fetch()
    }
}

pub(crate) enum Init {
    Result,
    Future(Operation),
}

impl Default for Init {
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
pub(crate) enum Storage {
    #[default]
    StArray,
    DyArray,
}

pub(crate) trait Property {}

impl Property for Binding {}
impl Property for Group {}
impl Property for Init {}
impl Property for Length {}
impl Property for Storage {}

pub(crate) trait Fetch: Property {
    type Value;
    fn fetch(&self) -> Self::Value;
}

impl Fetch for Binding {
    type Value = u32;
    fn fetch(&self) -> Self::Value {
        match self {
            Binding::Assigned(binding) => *binding,
            Binding::Hold => panic!(),
        }
    }
}

impl Fetch for Group {
    type Value = u32;
    fn fetch(&self) -> Self::Value {
        match self {
            Group::Base => 0,
            Group::Custom(group) => *group,
        }
    }
}

impl Fetch for Length {
    type Value = usize;
    fn fetch(&self) -> Self::Value {
        match self {
            Length::Sized(length) => *length,
            Length::Unsized => 0,
        }
    }
}

macro_rules! impl_display {
    ($($property:ty, )*) => {$(
        impl Display for $property {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.fetch())
            }
        }
    )*}
}

impl_display! {
    Binding,
    Group,
    Length,
}

#[derive(Clone, Copy, Default)]
pub(crate) enum Layout {
    #[default]
    Init,
    Future,
    Dyn,
}

pub(crate) struct Properties {
    pub binding: Binding,
    pub group: Group,
    pub init: Init,
    pub length: Length,
    pub storage: Storage,
}

impl Properties {
    pub fn construct(layout: Layout, length: usize, op: Option<Operation>) -> Self {
        let props = match layout {
            Layout::Init => Self {
                binding: Binding::default(),
                group: Group::default(),
                init: Init::default(),
                length: Length::Sized(length),
                storage: Storage::default(),
            },
            Layout::Future => Self {
                binding: Binding::Hold,
                group: Group::default(),
                init: {
                    if let Some(op) = op {
                        Init::Future(op)
                    } else {
                        panic!()
                    }
                },
                length: Length::Sized(length),
                storage: Storage::default(),
            },
            Layout::Dyn => Self {
                binding: Binding::Hold,
                group: Group::default(),
                init: Init::default(),
                length: {
                    if length != 0 {
                        Length::Sized(length)
                    } else {
                        Length::Unsized
                    }
                },
                storage: Storage::DyArray,
            },
        };

        props
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
    pub props: Properties,
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

    pub fn bind_future(length: usize, op: Operation) -> Result<Self, wgpu::Error> {
        let props = Properties::construct(Layout::Future, length, Some(op));

        let bundle = Self {
            buffer: BufferHolder::new(),
            props,
        };

        Ok(bundle)
    }

    pub fn bind_dyn(length: usize) -> Result<Self, wgpu::Error> {
        let props = Properties::construct(Layout::Dyn, length, None);

        let bundle = Self {
            buffer: BufferHolder::new(),
            props,
        };

        Ok(bundle)
    }
}
