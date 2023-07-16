use wgpu;

use crate::internals::Handler;

pub struct Instance {
    handler: Handler,
}

impl Instance {
    pub fn init() -> Result<Self, wgpu::Error> {
        let handler = Handler::request()?;

        let instance = Self { handler };

        Ok(instance)
    }
}
