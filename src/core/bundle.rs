use {
    std::{
        any,
        fmt::{self, Display, Formatter},
        marker::PhantomData,
        mem::MaybeUninit,
        ops::{Deref, DerefMut},
        sync::atomic::{AtomicU32, Ordering},
    },
    wgpu,
};

use crate::{
    internals::Buffer,
    types::{Packet, SupportedPacket},
};

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

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum Dimensions {
    Sized(Vec<u32>),
    Unsized,
}

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
pub(crate) enum Relay {
    #[default]
    Skip,
    Operate,
    Map,
}

impl Relay {
    #[inline]
    fn exhausted(&self) -> bool {
        match self {
            Relay::Skip => true,
            Relay::Operate => false,
            Relay::Map => true,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub(crate) enum Storage {
    #[default]
    StArray,
    DyArray,
}

pub(crate) trait Property {}

macro_rules! impl_property {
    ($($property:ty, )*) => {$(
        impl Property for $property {}
    )*}
}

impl_property! {
    Binding,
    Dimensions,
    Group,
    Relay,
    Storage,
}

pub(crate) trait Fetch: Property {
    type Value;
    fn fetch(&self) -> Self::Value;
}

impl Fetch for Binding {
    type Value = u32;

    #[inline]
    fn fetch(&self) -> Self::Value {
        match self {
            Binding::Assigned(binding) => *binding,
            Binding::Hold => panic!(),
        }
    }
}

impl Fetch for Dimensions {
    type Value = u32;

    #[inline]
    fn fetch(&self) -> Self::Value {
        match self {
            Dimensions::Sized(dims) => dims.iter().sum(),
            Dimensions::Unsized => 0,
        }
    }
}

impl Fetch for Group {
    type Value = u32;

    #[inline]
    fn fetch(&self) -> Self::Value {
        match self {
            Group::Base => 0,
            Group::Custom(group) => *group,
        }
    }
}

macro_rules! impl_display {
    ($($property:ty, )*) => {$(
        impl Display for $property {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "{:?}", self.fetch())
            }
        }
    )*}
}

impl_display! {
    Binding,
    Dimensions,
    Group,
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
    pub dims: Dimensions,
    pub group: Group,
    pub relay: Relay,
    pub storage: Storage,
}

impl Properties {
    pub fn construct<T>(layout: Layout, dims: Vec<u32>) -> Self
    where
        Packet<T>: SupportedPacket,
    {
        let props = match layout {
            Layout::Init => Self {
                dims: Dimensions::Sized(dims),
                ..Default::default()
            },
            Layout::Future => Self {
                binding: Binding::Hold,
                dims: Dimensions::Sized(dims),
                relay: Relay::Operate,
                ..Default::default()
            },
            Layout::Dyn => Self {
                binding: Binding::Hold,
                storage: Storage::DyArray,
                ..Default::default()
            },
        };

        props
    }
}

impl Default for Properties {
    fn default() -> Self {
        Self {
            binding: Binding::default(),
            dims: Dimensions::Unsized,
            group: Group::default(),
            relay: Relay::default(),
            storage: Storage::default(),
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

pub(crate) struct Bundle<T>
where
    Packet<T>: SupportedPacket,
{
    pub buffer: BufferHolder,
    pub layout: Layout,
    pub props: Properties,
    pub op: Option<Operation<T>>,

    target: PhantomData<T>,
}

impl<T> Bundle<T>
where
    Packet<T>: SupportedPacket,
{
    pub fn bind_init(dims: Vec<u32>) -> Result<Self, wgpu::Error> {
        let layout = Layout::default();
        let props = Properties::construct::<T>(layout, dims);

        let bundle = Self {
            buffer: BufferHolder::new(),
            layout,
            props,
            op: None,
            target: PhantomData,
        };

        Ok(bundle)
    }

    pub fn bind_future(dims: Vec<u32>, op: Operation<T>) -> Result<Self, wgpu::Error> {
        let layout = Layout::Future;
        let props = Properties::construct::<T>(layout, dims);

        let bundle = Self {
            buffer: BufferHolder::new(),
            layout,
            props,
            op: Some(op),
            target: PhantomData,
        };

        Ok(bundle)
    }

    pub fn bind_dyn(dims: Vec<u32>) -> Result<Self, wgpu::Error> {
        let layout = Layout::Dyn;
        let props = Properties::construct::<T>(layout, dims);

        let bundle = Self {
            buffer: BufferHolder::new(),
            layout,
            props,
            op: None,
            target: PhantomData,
        };

        Ok(bundle)
    }

    #[inline]
    pub fn ready(&self) -> bool {
        self.buffer.init
    }

    #[inline]
    pub fn typename(&self) -> &'static str {
        any::type_name::<T>()
    }

    /// Map to CPU and update if requested.
    fn map(&self) {}

    /// Retrieve values if dynamic.
    fn poll(&mut self) {
        match self.layout {
            Layout::Dyn => {}
            _ => {}
        }
    }
}
