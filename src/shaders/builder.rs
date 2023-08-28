#![allow(unused_imports)]

use std::{
    borrow::Cow,
    fmt::{Display, Formatter, Write},
    mem,
};

use super::{BundleShader, OperationShader};
use crate::core::{Bundle, Operation};

#[derive(Clone, Copy, Default)]
pub(crate) enum Phase {
    #[default]
    Headers,
    Calls,
    Open,
    Close,
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
}

#[derive(Clone, Copy)]
pub(crate) struct Workgroup(u32, u32, u32);

impl Default for Workgroup {
    fn default() -> Self {
        Self(1, 1, 1)
    }
}

pub(crate) trait ShaderCore {
    fn headers(&mut self, elements: &[&dyn BundleShader]);
    fn calls(&mut self, ops: &[&dyn OperationShader]);
    fn open(&mut self, workgroup: Workgroup);
    fn close(&mut self);
}

#[cfg(feature = "wsgl")]
impl ShaderCore for Module {
    fn headers(&mut self, bundles: &[&dyn BundleShader]) {
        let mut _headers = String::new();
        for bundle in bundles.iter() {
            _headers += bundle.tag().as_str();
        }
        self.phase = Phase::Calls;
    }

    fn calls(&mut self, ops: &[&dyn OperationShader]) {
        let mut _calls = String::new();
        for op in ops.iter() {}
    }

    fn open(&mut self, workgroup: Workgroup) {
        let Workgroup(x, y, z) = workgroup;
        let _open = format!(
            "@compute @workgroup_size({}, {}, {}) fn main(@builtin(global_invocation_id) index: vec3<u32>) {{"
            , x, y, z
        );
        self.phase = Phase::Close;
    }

    fn close(&mut self) {
        let _close = "}";
        self.phase = Phase::Ready;
    }
}
