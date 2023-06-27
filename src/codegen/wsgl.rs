use std::fmt::{Display, Formatter, Write};

use super::builder::Element;
use crate::api::{Bundle, VMemory, VState};

impl Element for Bundle {
    #[inline]
    fn tag(&self) -> String {
        match self.memory {
            VMemory::Static => {
                format!("group(self._group) binding(self._binding) var<storage, read_write> array<self._alias>")
            }
            VMemory::Runtime => {
                panic!()
            }
        }
    }
}
