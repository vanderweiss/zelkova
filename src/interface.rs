// Low level user API behind the toolkit

use {
    std::{
        collections::HashMap,
        default::Default,
    },
};

use super::shader::*;

// Buffers associated with toolkit models
pub struct Bundle<'a> {
    link: BufferEntry<'a>,
    valid: bool,
}

impl Bundle<'_> {
    pub fn pack() -> Self {
        
    }
}

// GPU memory layout
pub struct Layout<'a> {
    mapping: HashMap<u16, Bundle<'a>>,
}

impl Layout<'_> {
    pub fn schedule(&self) -> Bundle {
    }
}

pub enum NodePlacement {
    Head,
    Body,
    Tail,
}

pub struct Node<'a> {
    bundle: Bundle<'a>,
    placement: NodePlacement,
}

// Container for lazy execution
pub struct OperationContext<'a> {
    governor: &'static Governor,
    relative: ComputeContext<'a>,
}

impl OperationContext<'_> {
    pub fn pack() -> Self {

    }

    pub fn process(&self) {

    }
}

// Soon
pub struct GovernorOptions {
    local_modules: Option<&'static str>,
}

impl Default for GovernorOptions {
    fn default() -> Self {
        Self {
            local_modules: Some("/"),
        }
    }
}

// Middlepoint for any calls between users and wgpu
pub struct Governor {
    handler: Handler,
    options: GovernorOptions,
}

impl Governor {
    pub fn new(options: Option<GovernorOptions>) -> Self {
        Self {
            handler: Handler::request()?,
            options: options.unwrap_or_default(),
        }
    }
}
