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

    pub fn process(&self) {}
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
pub(crate) struct OperationTree<'c, 't: 'c, Factor>
where
    Factor: OperationFactor,
{
    nodes: Vec<OperationNode<'t, Factor>>,
    context: Option<ComputeContext<'c>>,
}

impl<'c, 't: 'c, Factor> OperationTree<'c, 't, Factor>
where
    Factor: OperationFactor,
{
    #[inline]
    pub fn create() -> Self {
        Self {
            nodes: Vec::<OperationNode<'_, Factor>>::new(),
            context: None,
        }
    }

    pub fn process(&self) {}
}

pub(crate) enum OperationSource {
    Toolkit(&'static str),
    Imported(&'static str, &'static str),
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub(crate) struct OperationType: u8 {
        const Arithmetic = 0b00000001;
        const Protected  = 0b00000010;
    }
}

// Container for lazy execution
pub(crate) struct OperationNode<'b, Factor>
where
    Factor: OperationFactor,
{
    bundles: Vec<&'b Factor>,
    source: OperationSource,
    ty: OperationType,
}

impl<'b, Factor> OperationNode<'b, Factor>
where
    Factor: OperationFactor,
{
    #[inline]
    pub fn create(source: OperationSource, ty: OperationType) -> Self {
        Self {
            bundles: Vec::<&'b Factor>::new(),
            source,
            ty,
        }
    }

    pub fn include(mut self, bundle: &'b Factor, context: Option<ComputeContext>) -> Self {
        match context {
            None => self.bundles.push(bundle),
            Some(_) => panic!(),
        };
        self
    }

    pub fn process(&self) {}
}

// generic way for working with nested ops and tensors
pub(crate) trait OperationFactor {
    type NodeFactor;
}

impl OperationFactor for Bundle {
    type NodeFactor = Self;
}

impl<'b, Factor> OperationFactor for OperationNode<'b, Factor>
where
    Factor: OperationFactor,
{
    type NodeFactor = Self;
}
