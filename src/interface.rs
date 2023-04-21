// Low level user API behind the toolkit

use {
    std::{
        collections::HashMap,
        default::Default,
    },
};

use super::shader::*;

pub struct Bundle<'a> {
    link: BufferEntry<'a>,
    valid: bool,
}

pub struct Layout<'a> {
    mapping: HashMap<u16, Bundle<'a>>,
}

impl Layout<'_> {
    pub fn schedule(&self) -> Bundle {
                
    }
}

pub enum Operation {

}

pub struct OperationContext<'a> {
    governor: &'static Governor,
    relative: ComputeContext<'a>,
}

impl OperationContext<'_> {
    pub fn process(&self) {

    }
}

pub struct GovernorOptions {
    local_modules: bool,
}

impl Default for GovernorOptions {
    fn default() -> Self {
        Self {
            local_modules: false,
        }
    }
}

pub struct Governor {
    handler: Handler,
    options: GovernorOptions,
}

impl Governor {
    pub fn new(options: Option<GovernorOptions>) -> Self {
        Self {
            handler: Handler::request()?,
            options: options.unwrap_or(GovernorOptions::default()),
        }
    }

    pub fn pack(&self) -> OperationContext {
        
    }
}
