// Low level user API behind the toolkit

use {std::collections::HashMap, wgpu};

use crate::codegen::{Buffer, Component, ComputeContext};

// Buffers associated with toolkit models, contiguous arrays mostly
pub(crate) struct Bundle {
    layout: &'static mut Layout,
    buffer: Buffer,
    valid: bool,
}

impl Bundle {
    pub fn bind<C: Component>(content: &[C], binding: u32) -> Result<&mut Self, wgpu::Error> {
        static mut layout: Layout = Layout::arrange();

        let ref mut bundle = unsafe {
            layout.insert(
                Self {
                    layout: &mut layout,
                    buffer: Buffer::bind::<_>(content, binding)?,
                    valid: false,
                },
                binding,
            )
        };

        Ok(bundle)
    }
}

// GPU memory layout in respect to Bundle containers
struct Layout {
    mapping: HashMap<u32, Bundle>,
}

impl Layout {
    #[inline]
    pub fn arrange() -> Self {
        Self {
            mapping: HashMap::new(),
        }
    }

    #[inline]
    pub fn insert(&mut self, bundle: Bundle, binding: u32) -> &Bundle {
        &self.mapping.insert(binding, bundle).unwrap()
    }

    pub fn recycle(&self) {}
}

// State saver for nodes and traversal on evaluation
pub(crate) struct OperationTree<'a> {
    nodes: Vec<OperationNode<'a>>,
    context: Option<ComputeContext<'a>>,
}

impl OperationTree<'_> {
    #[inline]
    pub fn create() -> Self {
        Self {
            nodes: Vec::<OperationNode>::new(),
            context: None,
        }
    }

    pub fn process(&self) {}
}

// Container for lazy execution
pub(crate) struct OperationNode<'a> {
    bundles: Vec<&'a Bundle>,
    parent: Option<OperationTree<'a>>,
}

impl OperationNode<'_> {
    #[inline]
    pub fn create() -> Self {
        Self {
            bundles: Vec::<&Bundle>::new(),
            parent: None,
        }
    }

    pub fn include(&mut self, bundle: &Bundle) -> &mut Self {
        match self.parent {
            None => self.bundles.push(bundle),
            Some(_) => panic!(),
        };
        self
    }

    pub fn init(&mut self, parent: OperationTree) {}
}
