use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Write},
};

use super::builder::{Element, Operation};
use crate::api::{Bundle, Node, NodeTree, NodeType, OperationFactor, VMemory, VState};

impl Element for Bundle {
    fn alias(&self) -> String {
        format!("obj{0}", self._binding)
    }

    fn mode(&self) -> String {
        let implies = HashMap::from([("uniform", "read"), ("storage", "read_write")]);
        let mut format = String::new();

        format.push_str(implies[self.space().as_str()]);

        format
    }

    fn space(&self) -> String {
        let spaces = [("uniform", 1 << 6), ("storage", 1 << 7)];
        let mut format = String::new();

        if let Some((space, _)) = spaces.iter().find(|(_, v)| self.buffer.bits() & v != 0) {
            format.push_str(space);
        }

        format
    }

    fn tag(&self) -> String {
        match self.memory {
            VMemory::Static => {
                format!(
                    "group({0}) binding({1}) var<{2}, {3}> {4}: array<{5}>",
                    self._group,
                    self._binding,
                    self.space(),
                    self.mode(),
                    self.alias(),
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
