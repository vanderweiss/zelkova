pub(crate) mod shader;

pub(crate) use self::shader::BufferEntry;
pub(crate) use self::shader::Component;
pub(crate) use self::shader::ComputeContext;

pub(crate) type _Bty = u8; // Buffer type - from primitive slices
pub(crate) type _Tty = u32; // Tracker type - layout util
pub(crate) type _Pty = u64; // Position type - set boundaries
