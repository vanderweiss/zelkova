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
use crate::internals::ComputeContext;

use super::Bundle;

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
pub(crate) struct NodeTree<'c, 't: 'c, Factor>
where
    Factor: OperationFactor,
{
    origin: Node<'t, Factor>,
    depth: TreeDepth,
    context: Option<ComputeContext<'c>>,
}

impl<'c, 't: 'c, Factor> NodeTree<'c, 't, Factor>
where
    Factor: OperationFactor,
{
    pub fn create(origin: Node<'t, Factor>) -> Self {
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

pub(crate) enum NodeType {
    Arithmetic,
}

pub(crate) struct Node<'b, Factor>
where
    Factor: OperationFactor,
{
    pub factors: Vec<&'b Factor>,
    pub source: ShaderSource,
    pub ty: NodeType,
}

impl<'b, Factor> Node<'b, Factor>
where
    Factor: OperationFactor,
{
    #[inline]
    pub fn create(source: ShaderSource, ty: NodeType) -> Self {
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

    pub fn uphold(&self, tree: &mut NodeTree<'_, 'b, Factor>) {}

    pub fn propagate(&self, tree: &mut NodeTree<'_, 'b, Factor>) {
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

impl<'b, Factor> OperationFactor for Node<'b, Factor>
where
    Factor: OperationFactor,
{
    type NodeFactor = Self;

    #[inline]
    fn resolved(&self) -> bool {
        false
    }
}
