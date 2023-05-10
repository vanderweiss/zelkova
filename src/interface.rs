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
    entry: BufferEntry<'a>,
    valid: bool,
}

impl Bundle<'_> {
    pub fn bind<T: Element, const N: usize>(content: &[T; N], id: u32) -> Self {
        let entry = BufferEntry::bind(content, id).unwrap();

        Self { entry, valid: true }
    }
}

// GPU memory layout in respect to Bundle containers
pub struct Layout<'a> {
    mapping: HashMap<u32, Bundle<'a>>,
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
    relative: ComputeContext<'a>,
}

impl OperationContext<'_> {
    // Returns Self
    pub fn pack() {}
    // Returns Node
    pub fn process(&self) {}
}
