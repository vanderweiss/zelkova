#![allow(unused_imports)]

use std::{
    borrow::Cow,
    fmt::{Display, Formatter, Write},
    mem,
};

use crate::core::Operation;

use super::Element;

#[derive(Clone, Copy, Default)]
pub(crate) enum Phase {
    #[default]
    Headers,
    Open,
    Calls,
    Close,
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

pub(crate) trait Entry {
    fn headers(&mut self, elements: &[Box<dyn Element>]);
    fn open(&mut self, workgroup: Workgroup);
    fn close(&mut self);
}

#[cfg(feature = "wsgl")]
impl Entry for Module {
    fn headers(&mut self, elements: &[Box<dyn Element>]) {
        let mut _headers = String::new();
        for element in elements.iter() {
            _headers += element.tag().as_str();
        }
    }

    fn open(&mut self, workgroup: Workgroup) {
        let Workgroup(x, y, z) = workgroup;
        let _open = format!("@compute @workgroup_size({}, {}, {}) fn main() {{", x, y, z);
        self.phase = Phase::Open;
    }

    fn close(&mut self) {
        let _close = "}";
        self.phase = Phase::Close;
    }
}
