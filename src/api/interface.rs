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

use crate::codegen::Builder;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TreeStatus {
    Pending,
    Ready,
}

impl TreeStatus {
    #[inline]
    fn ok(self) -> bool {
        self == Self::Ready
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TreeDepth {
    Singular,
    Layered(u32, TreeStatus),
}

impl TreeDepth {
    fn register(&self) {
        match self {
            Self::Singular => {
                panic!()
            }
            Self::Layered(mut depth, status) => {
                status.ok().then(|| depth += 1);
            }
        }
    }
}

// State saver for nodes and traversal on evaluation
pub(crate) struct OperationTree<'c, 't: 'c, Factor>
where
    Factor: OperationFactor,
{
    origin: OperationNode<'t, Factor>,
    depth: TreeDepth,
    context: Option<ComputeContext<'c>>,
}

impl<'c, 't: 'c, Factor> OperationTree<'c, 't, Factor>
where
    Factor: OperationFactor,
{
    pub fn create(origin: OperationNode<'t, Factor>) -> Self {
        let depth = {
            if origin.resolved() {
                TreeDepth::Singular
            } else {
                TreeDepth::Layered(1, TreeStatus::Pending)
            }
        };
        Self {
            origin,
            depth,
            context: None,
        }
    }

    /// If the origin contains a recursive nested operation, this serves as the startpoint. Otherwise resolve and return.
    /// Operation hinting will be a thing later on for more efficient codegen.
    pub fn traverse(&self) {}
}

pub(crate) enum ShaderSource {
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

pub(crate) struct OperationNode<'b, Factor>
where
    Factor: OperationFactor,
{
    factors: Vec<&'b Factor>,
    source: ShaderSource,
    ty: OperationType,
}

impl<'b, Factor> OperationNode<'b, Factor>
where
    Factor: OperationFactor,
{
    #[inline]
    pub fn create(source: ShaderSource, ty: OperationType) -> Self {
        Self {
            factors: Vec::<&'b Factor>::new(),
            source,
            ty,
        }
    }

    pub fn include(mut self, factor: &'b Factor, context: Option<ComputeContext>) -> Self {
        match context {
            None => self.factors.push(factor),
            Some(_) => panic!(),
        };
        self
    }

    pub fn uphold(&self, tree: &mut OperationTree<'_, 'b, Factor>) {}

    pub fn propagate(&self, tree: &mut OperationTree<'_, 'b, Factor>) {
        match self.resolved() {
            true => {}
            false => {}
        }
    }
}

pub(crate) trait OperationFactor {
    type NodeFactor;

    fn resolved(&self) -> bool;
}

impl OperationFactor for Bundle {
    type NodeFactor = Self;

    #[inline]
    fn resolved(&self) -> bool {
        true
    }
}

impl<'b, Factor> OperationFactor for OperationNode<'b, Factor>
where
    Factor: OperationFactor,
{
    type NodeFactor = Self;

    #[inline]
    fn resolved(&self) -> bool {
        false
    }
}
