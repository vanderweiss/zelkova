// Low level user API behind the toolkit

use {
    std::{
        collections::HashMap,
        default::Default,
    },
};

use super::shader::*;

// Buffers associated with toolkit models, contiguous arrays mostly
pub struct Bundle<'a> {
    binded: bool, 
    link: BufferEntry<'a>,
}

impl Bundle<'_> {
    // Returns Self
    pub fn pack() {
        
    }
}

// GPU memory layout in respect to Bundle containers
pub struct Layout<'a> {
    mapping: HashMap<u16, Bundle<'a>>,
}

impl Layout<'_> {
    pub fn create() -> Self {
       Self {
            mapping: HashMap::new(),
       } 
    }
    // Returns Bundle
    pub fn schedule(&self) {
        
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
    governor: &'static Governor<'static>,
    relative: ComputeContext<'a>,
}

impl OperationContext<'_> {
    // Returns Self
    pub fn pack() {
        
    }
    // Returns Node
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
pub struct Governor<'a> {
    handler: Handler,
    layout: Layout<'a>,
    options: GovernorOptions,
}

impl Governor<'_> {
    pub fn new(options: Option<GovernorOptions>) -> Self {
        Self {
            handler: Handler::request().unwrap(),
            layout: Layout::create(),
            options: options.unwrap_or_default(),
        }
    }
}
