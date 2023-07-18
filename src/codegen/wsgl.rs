use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Write},
};

use super::builder::{Element, Operation};
use crate::api::{Bundle, Memory, Node, OperationFactor, State};

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
        match self.props.state {
            State::Prepared => {
                let pre = format!(
                    "group({0}) binding({1}) var<{2}, {3}> tsr{2}>",
                    self.props.group,
                    self.props.binding,
                    self.space(),
                    self.mode(),
                );

                let post = match self.props.memory {
                    Memory::Static => {
                        format!(
                            "{0}: array<{1}, {2}>",
                            pre, self.props.alias, self.props.count
                        )
                    }
                    Memory::Runtime => {
                        format!("{0}: array<{1}>", pre, self.props.alias)
                    }
                };

                post
            }
            State::Binded => {
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
