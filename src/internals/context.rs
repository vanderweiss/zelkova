use {std::borrow::Cow, wgpu};

use super::Handler;

pub(crate) struct ComputeContext<'a> {
    bindgroup: wgpu::BindGroup,
    module: wgpu::ShaderModule,
    pass: wgpu::ComputePass<'a>,
    pipeline: wgpu::ComputePipeline,
}

/*
impl ComputeContext<'_> {
    pub fn pack(
        handler: Handler,
        bindgroup: wgpu::BindGroup,
        _module: Cow<'_, str>,
    ) -> Result<Self, wgpu::Error> {
        let (module, pipeline) = handler
            .load_module(_module)
            .expect("Processed corrupt module.");

        let pass = handler.start_pass()?;

        let context = Self {
            bindgroup,
            module,
            pass,
            pipeline,
        };

        Ok(context)
    }
    pub fn run(&self) {}
}
*/
