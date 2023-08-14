#![allow(unused_imports)]

use std::{
    borrow::Cow,
    fmt::{Display, Formatter, Write},
};

use super::Element;

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

    // Format before converting to module
    #[inline]
    pub fn wrap(&self) -> Cow<'_, str> {
        Cow::from(self.module.as_str())
    }
}
