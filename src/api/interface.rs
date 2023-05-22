// Low level user API behind the toolkit

use {std::collections::HashMap, wgpu};

use crate::codegen::{BufferEntry, Component, ComputeContext, _Tty};

// Buffers associated with toolkit models, contiguous arrays mostly
pub(crate) struct Bundle {
    layout: &'static mut Layout,
    entry: BufferEntry,
    valid: bool,
}

impl Bundle {
    pub fn bind<C: Component, const N: usize>(
        content: &[C; N],
        index: _Tty,
    ) -> Result<&mut Self, wgpu::Error> {
        static mut layout: Layout = Layout::arrange();

        let ref mut bundle = unsafe {
            layout.insert(
                Self {
                    layout: &mut layout,
                    entry: BufferEntry::bind::<_, N>(content, index)?,
                    valid: false,
                },
                index,
            )
        };

        Ok(bundle)
    }
}

// GPU memory layout in respect to Bundle containers
struct Layout {
    mapping: HashMap<_Tty, Bundle>,
}

impl Layout {
    #[must_use]
    #[inline]
    pub fn arrange() -> Self {
        Self {
            mapping: HashMap::new(),
        }
    }

    pub fn init() {}

    pub fn insert(&mut self, bundle: Bundle, index: _Tty) -> &Bundle {
        &self.mapping.insert(index, bundle).unwrap()
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
