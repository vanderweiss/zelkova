// Low level user API behind the toolkit

use {
    std::{
        collections::HashMap,
        ptr,
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
    pub fn bind<C: Component>(content: &[C], binding: u32) -> Result<&Self, wgpu::Error> {
        let layout = Layout::arrange();

        let bundle = unsafe {
            (*layout).insert(
                Self {
                    buffer: Buffer::bind::<_>(content, binding)?,
                    state: State::new(),
                },
                binding,
            )
        };

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
struct Layout {
    mapping: HashMap<u32, Bundle>,
}

impl Layout {
    #[inline]
    fn _arrange() -> Self {
        Self {
            mapping: HashMap::new(),
        }
    }

    pub fn arrange() -> *mut Self {
        static _layout: LazyLock<Mutex<Layout>> = LazyLock::new(|| Mutex::new(Layout::_arrange()));
        _layout
            .lock()
            .as_deref_mut()
            .map(|r| ptr::from_mut(r))
            .unwrap()
    }

    #[inline]
    pub unsafe fn insert(&mut self, bundle: Bundle, binding: u32) -> &Bundle {
        self.mapping.try_insert(binding, bundle).unwrap_unchecked()
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

pub(crate) enum OperationSource {
    Toolkit(&'static str),
    Imported(&'static str, &'static str),
}

// Container for lazy execution
pub(crate) struct OperationNode<'b> {
    bundles: Vec<&'b Bundle>,
    source: OperationSource,
}

impl<'b> OperationNode<'b> {
    #[inline]
    pub fn create(source: OperationSource) -> Self {
        Self {
            bundles: Vec::<&'b Bundle>::new(),
            source,
        }
    }

    pub fn include(mut self, bundle: &'b Bundle, context: Option<ComputeContext>) -> Self {
        match context {
            None => self.bundles.push(bundle),
            Some(_) => panic!(),
        };
        self
    }

    pub fn process(&self) {}
}
