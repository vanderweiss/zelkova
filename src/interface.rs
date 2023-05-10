// Low level user API behind the toolkit

use std::{collections::HashMap, default::Default};

use super::shader::*;

// Buffers associated with toolkit models, contiguous arrays mostly
pub struct Bundle {
    entry: BufferEntry,
    valid: bool,
}

impl Bundle {
    pub fn bind<C: Component, const N: usize>(content: &[C; N], id: u32) -> Self {
        let entry = BufferEntry::bind::<_, N>(content, id).unwrap();

        Self { entry, valid: true }
    }
}

// GPU memory layout in respect to Bundle containers
pub struct Layout {
    mapping: HashMap<u32, Bundle>,
}

impl Layout {
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
