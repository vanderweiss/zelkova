use std::fmt::{Display, Formatter, Write};

use super::builder::Element;
use crate::api::{Bundle, VMemory, VState};

impl Element for Bundle {
    fn mode(&self) -> String {
        let modes = [("read", 1 << 0), ("write", 1 << 1)];
        let mut format = String::new();

        if modes.iter().all(|(_, v)| self.buffer.bits() & v != 0) {
            format.push_str("read_write");
        } else {
            if let Some((mode, _)) = modes.iter().find(|(_, v)| self.buffer.bits() & v != 0) {
                format.push_str(mode);
            }
        }

        format
    }

    fn space(&self) -> String {
        let spaces = [("uniform", 1 << 6), ("storage", 1 << 7)];
        let mut format = String::new();

        if let Some((space, _)) = spaces.iter().find(|(_, v)| self.buffer.bits() & v != 0) {
            format.push_str(space);
        }

        format
    }

    fn tag(&self) -> String {
        match self.memory {
            VMemory::Static => {
                format!(
                    "group({0}) binding({1}) var<{2}, {3}> array<{4}>",
                    self._group,
                    self._binding,
                    self.space(),
                    self.mode(),
                    self._alias,
                )
            }
            VMemory::Runtime => {
                panic!()
            }
        }
    }
}
