#![allow(unused_imports)]

use std::{
    borrow::Cow,
    fmt::{self, Display, Write},
    mem,
};

use super::{BundleShader, OperationShader};
use crate::core::{Bundle, Operation};

#[derive(Clone, Copy, Default)]
pub(crate) enum Phase {
    #[default]
    Headers,
    Compute,
    Ready,
}

pub(crate) struct Module {
    content: String,
    phase: Phase,
}

impl Module {
    #[inline]
    pub fn new() -> Self {
        Self {
            content: String::new(),
            phase: Phase::default(),
        }
    }

    #[inline]
    pub fn wrap(&self) -> Cow<'_, str> {
        Cow::from(self.content.as_str())
    }

    #[inline]
    pub fn write<S: AsRef<str>>(&mut self, input: S) {
        writeln!(&self.content, "{}", input);
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Directive {
    F16,
}

pub(crate) trait ShaderCore {
    fn insert_directive(&mut self, directive: Directive);
    fn insert_header(&mut self, elements: &dyn BundleShader);
    fn insert_compute(&mut self, op: &dyn OperationShader);
}

#[cfg(feature = "wsgl")]
impl ShaderCore for Module {
    fn insert_directive(&mut self, directive: Directive) {
        let extension = match directive {
            Directive::F16 => "shader-f16",
        };
        self.write(extension);
    }
    fn insert_header(&mut self, bundle: &dyn BundleShader) {}

    fn insert_compute(&mut self, op: &dyn OperationShader) {}
}
