#![allow(unused_imports)]

use std::{
    borrow::Cow,
    io::{Result, Write},
};

pub(crate) struct Builder {
    module: String,
}

impl Builder {
    #[inline]
    pub fn new() -> Self {
        Self {
            module: String::new(),
        }
    }

    #[inline]
    pub fn wrap(&self) -> Cow<'_, str> {
        Cow::from(self.module)
    }
}

pub(crate) trait Factor {}

struct Expr {}
struct Scope {}
