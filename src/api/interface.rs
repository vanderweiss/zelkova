// Low level user API behind the toolkit

use {
    std::{
        collections::HashMap,
        sync::{LazyLock, Mutex},
    },
    wgpu,
};

use crate::codegen::{Buffer, Component, ComputeContext};

// Buffers associated with toolkit models, contiguous arrays mostly
pub(crate) struct Bundle {
    buffer: Buffer,
    state: State,
}

impl Bundle {
    pub fn bind<C: Component>(content: &[C], binding: u32) -> Result<Self, wgpu::Error> {
        let ref mut layout = Layout::arrange();

        let bundle = Self {
            buffer: Buffer::bind::<_>(content, binding)?,
            state: State::new(),
        };

        layout.insert(&bundle, binding);

        Ok(bundle)
    }
}

pub(crate) struct State {
    // protected: bool,
    // shared: bool,
    valid: bool,
}

impl State {
    #[inline]
    pub fn new() -> Self {
        Self { valid: false }
    }
}

// GPU memory layout in respect to Bundle containers
struct Layout<'b> {
    mapping: HashMap<u32, &'b Bundle>,
}

impl<'b> Layout<'b> {
    #[inline]
    fn _arrange() -> Self {
        Self {
            mapping: HashMap::new(),
        }
    }

    pub fn arrange() -> &'static Self {
        static _layout: LazyLock<Layout> = LazyLock::new(|| Layout::_arrange());
        &_layout
    }

    #[inline]
    pub fn insert(&mut self, bundle: &'b Bundle, binding: u32) {
        self.mapping.insert(binding, bundle).unwrap();
    }

    pub fn recycle(&self) {}
}

// State saver for nodes and traversal on evaluation
pub(crate) struct OperationTree<'c, 't: 'c> {
    nodes: Vec<OperationNode<'t>>,
    context: Option<ComputeContext<'c>>,
}

impl<'c, 't: 'c> OperationTree<'c, 't> {
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
pub(crate) struct OperationNode<'b> {
    bundles: Vec<&'b Bundle>,
}

impl<'b> OperationNode<'b> {
    #[inline]
    pub fn create() -> Self {
        Self {
            bundles: Vec::<&'b Bundle>::new(),
        }
    }

    pub fn include(mut self, bundle: &'b Bundle, context: Option<ComputeContext>) -> Self {
        match context {
            None => self.bundles.push(bundle),
            Some(_) => panic!(),
        };
        self
    }
}
