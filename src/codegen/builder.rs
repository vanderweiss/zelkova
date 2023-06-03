#![allow(unused_imports)]

use std::borrow::Cow;

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

    // Generate buffer bindings
    pub fn headers(&mut self) {}

    // Format before converting to module
    #[inline]
    pub fn wrap(&self) -> Cow<'_, str> {
        Cow::from(self.module.as_str())
    }
}

pub(crate) type Fragment = &'static str;