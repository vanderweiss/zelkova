use std::fmt::{Display, Formatter, Write};

use super::builder::Element;
use crate::api::{Bundle, VMemory, VState};

impl Element for Bundle {
    #[inline]
    fn access(&self) -> String {
        self.buffer.permissions()
    }

    #[inline]
    fn tag(&self) -> String {
        match self.memory {
            VMemory::Static => {
                format!(
                    "group({0}) binding({1}) var<{2}> array<{3}>",
                    self._group,
                    self._binding,
                    self.access(),
                    self._alias,
                )
            }
            VMemory::Runtime => {
                panic!()
            }
        }
    }
}
