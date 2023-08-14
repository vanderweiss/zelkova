use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Write},
};

use crate::core::Bundle;

pub(crate) trait Element {
    fn mode(&self) -> &'static str;
    fn space(&self) -> &'static str;
    fn specifier(&self) -> (&'static str, &'static str);
    fn tag(&mut self) -> String;
}

#[cfg(feature = "wsgl")]
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

    fn tag(&mut self) -> String {
        let pre = format!(
            "group({0}) binding({1}) var<{2}, {3}> tsr{2}>",
            self.props.group(),
            self.props.binding(),
            self.space(),
            self.mode(),
        );

        let post = {
            if self.props.ready() {
                format!(
                    "{0}: array<{1}, {2}>",
                    pre,
                    self.props.alias(),
                    self.props.length(),
                )
            } else {
                format!("{0}: array<{1}>", pre, self.props.alias())
            }
        };

        post
    }
}
