// Low level user API behind the toolkit

use std::collections::HashMap;

use crate::codegen::{BufferEntry, Component, ComputeContext};

// Buffers associated with toolkit models, contiguous arrays mostly
pub struct Bundle {
    entry: BufferEntry,
    valid: bool,
}

impl Bundle {
    pub fn bind<C: Component, const N: usize>(content: &[C; N], index: u32) -> Self {
        static mut layout: Layout = Layout::arrange();
        let bundle = Self {
            entry: BufferEntry::bind::<_, N>(content, index).unwrap(),
            valid: false,
        };
        bundle
    }
}

// GPU memory layout in respect to Bundle containers
pub struct Layout<'a> {
    mapping: HashMap<u32, &'a Bundle>,
}

impl Layout<'_> {
    pub fn arrange() -> Self {
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
