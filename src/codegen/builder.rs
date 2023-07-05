#![allow(unused_imports)]

use std::{
    borrow::Cow,
    fmt::{Display, Formatter, Write},
};

use crate::api::Bundle;

/// Bundle representation for codegen purposes.
pub(crate) trait Element {
    fn mode(&self) -> String;
    fn space(&self) -> String;
    fn tag(&self) -> String;
}

pub(crate) struct Pointer(u32, u32);

pub(crate) struct Builder {
    module: String,
    pointer: Pointer,
}

impl Builder {
    #[inline]
    pub fn new() -> Self {
        Self {
            module: String::new(),
            pointer: Pointer(0, 0),
        }
    }

    // Generate buffer bindings
    pub fn headers(&mut self, bundle: &Bundle) {}

    // Format before converting to module
    #[inline]
    pub fn wrap(&self) -> Cow<'_, str> {
        Cow::from(self.module.as_str())
    }
}
