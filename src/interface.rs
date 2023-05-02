// Low level user API behind the toolkit

use {
    bytemuck::NoUninit,
    std::{collections::HashMap, default::Default},
};

use super::shader::*;

mod element {
    pub trait Sealed {}
}

pub trait Element: element::Sealed + NoUninit {}

macro_rules! impl_element {
    ($($ident:ident)*) => {$(
        impl Element for $ident {}
        impl element::Sealed for $ident {}
    )*}
}

impl_element! {
    u16 u32 u64
    i16 i32 i64
    f32 f64
}

// Buffers associated with toolkit models, contiguous arrays mostly
pub struct Bundle<'a> {
    binded: bool,
    link: BufferEntry<'a>,
}

impl Bundle<'_> {
    pub fn bind<T: Element, const N: usize>(content: &[T; N], id: u32) -> Self {
        Self {
            binded: true,
            link: BufferEntry::bind(content, id),
        }
    }
}

pub enum NodePosition {
    Head,
    Body,
    Tail,
}

pub struct Node<'a> {
    bundle: Bundle<'a>,
    position: NodePosition,
}

// GPU memory layout in respect to Bundle containers
pub struct Layout<'a> {
    mapping: HashMap<u16, Node<'a>>,
}

impl Layout<'_> {
    pub fn create() -> Self {
        Self {
            mapping: HashMap::new(),
        }
    }

    /*pub fn schedule<T: Element, const N: usize>(&self, container: [T; N]) -> Bundle {

    let entry = BufferEntry::bind(&container);

        Bundle {
            binded: true,
            link: entry,
        }
    }*/
}

// Container for lazy execution
pub struct OperationContext<'a> {
    governor: &'static Governor<'static>,
    relative: ComputeContext<'a>,
}

impl OperationContext<'_> {
    // Returns Self
    pub fn pack() {}
    // Returns Node
    pub fn process(&self) {}
}

// Soon
pub struct GovernorOptions {
    local_modules: Option<&'static str>,
}

impl Default for GovernorOptions {
    fn default() -> Self {
        Self {
            local_modules: Some("/"),
        }
    }
}

// Middlepoint for any calls between users and wgpu
pub struct Governor<'a> {
    handler: Handler,
    layout: Layout<'a>,
    options: GovernorOptions,
}

impl Governor<'_> {
    pub fn new(options: Option<GovernorOptions>) -> Self {
        Self {
            handler: Handler::request().unwrap(),
            layout: Layout::create(),
            options: options.unwrap_or_default(),
        }
    }
}
