use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Write},
};

use super::builder::{Element, Operation};
use crate::api::{Bundle, Node, NodeTree, NodeType, OperationFactor, VMemory, VState};

impl Element for Bundle {
    fn mode(&self) -> &'static str {
        self.specifier().1
    }

    fn space(&self) -> &'static str {
        self.specifier().0
    }

    fn specifier(&self) -> (&'static str, &'static str) {
        if self.buffer.is_uniform() {
            ("uniform", "read")
        } else if self.buffer.is_storage() {
            ("storage", "read_write")
        } else {
            panic!()
        }
    }

    fn tag(&self) -> String {
        match self.memory {
            VMemory::Static => {
                format!(
                    "group({0}) binding({1}) var<{2}, {3}> tsr{4}: array<{5}>",
                    self._group,
                    self._binding,
                    self.space(),
                    self.mode(),
                    self._binding,
                    self._alias,
                )
            }
            VMemory::Runtime => {
                panic!()
            }
        }
    }
}

impl<'b, Factor> Operation for Node<'b, Factor>
where
    Factor: OperationFactor,
{
    fn expand(&self) -> String {
        let mut format = String::new();
        format
    }
}
