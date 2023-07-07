//pub(crate) mod error;
pub(crate) mod core;
pub(crate) mod interface;

pub(crate) use self::core::{Bundle, VMemory, VState};
pub(crate) use self::interface::{Node, NodeTree, NodeType, OperationFactor, ShaderSource};
