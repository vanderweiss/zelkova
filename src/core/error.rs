use wgpu;

#[derive(Clone, Debug)]
pub(crate) enum ZkError {
    Wgpu(wgpu::Error),
}

impl From<wgpu::Error> for ZkError {
    fn from(error: wgpu::Error) -> Self {
        ZkError::Wgpu(error)
    }
}
