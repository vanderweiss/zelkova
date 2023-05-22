// wip custom error type, still unsure whether to standarize it

use std::{fmt, result};

#[derive(Clone, Debug)]
pub(crate) enum Error {
    Toolkit,
    Wgpu,
}

pub(crate) type ResultTk<T> = result::Result<T, Error>;
