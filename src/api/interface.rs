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

    #[inline]
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
pub(crate) struct OperationMap<'a> {
    bundles: Vec<&'a Bundle>,
    context: Option<ComputeContext<'a>>,
}

impl OperationMap<'_> {
    pub fn create() -> Self {
        Self {
            bundles: Vec::<&Bundle>::new(),
            context: None,
        }
    }

    pub fn include(&mut self, bundle: &Bundle) -> &mut Self {
        match self.context {
            None => self.bundles.push(bundle),
            Some(_) => panic!(),
        };
        self
    }

    pub fn process(&self) {}
}
