// Low level user API behind the toolkit

use {
    bitflags::bitflags,
    std::{
        collections::HashMap,
        ptr,
        sync::{LazyLock, Mutex},
    },
    wgpu,
};

use crate::internals::{Buffer, Component, ComputeContext};

// Buffers associated with toolkit models, contiguous arrays mostly
pub(crate) struct Bundle {
    buffer: Buffer,
}

impl Bundle {
    pub fn bind<C: Component>(content: &[C], binding: u32) -> Result<&Self, wgpu::Error> {
        let layout = Layout::arrange();

        let bundle = unsafe {
            (*layout).insert(
                Self {
                    buffer: Buffer::bind::<_>(content, binding)?,
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
    ty: OperationType,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub(crate) struct OperationType: u8 {
        const Arithmetic = 0b00000001;
        const Protected  = 0b00000010;
        const Singular   = 0b00000100;
    }
}

impl<'b> OperationNode<'b> {
    #[inline]
    pub fn create(source: OperationSource, ty: OperationType) -> Self {
        Self {
            bundles: Vec::<&'b Bundle>::new(),
            source,
            ty,
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

pub(crate) trait OperationFuture {
    type Packed;
}
