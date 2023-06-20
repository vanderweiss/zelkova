use {
    std::{
        collections::HashMap,
        ptr,
        sync::{LazyLock, Mutex},
    },
    wgpu,
};

use crate::internals::{Buffer, Component};

pub(crate) enum VState {
    Allocated,
    Binded,
    Prepared,
}

/// Handler for wgpu buffer representations. A thin wrapper that aids with codegen and operations performed on them.
/// Shouldn't be directly intantiated.
pub(crate) struct Bundle {
    buffer: Buffer,
    state: VState,
}

impl Bundle {
    pub fn bind<C: Component>(content: &[C], binding: u32) -> Result<&Self, wgpu::Error> {
        let layout = Layout::arrange();

        let bundle = unsafe {
            (*layout).insert(
                Self {
                    buffer: Buffer::bind::<_>(content, binding)?,
                    state: VState::Binded,
                },
                binding,
            )
        };

        Ok(bundle)
    }
}

struct Layout {
    mapping: HashMap<u32, Bundle>,
}

impl Layout {
    #[inline]
    fn _arrange() -> Self {
        Self {
            mapping: HashMap::new(),
        }
    }

    pub fn arrange() -> *mut Self {
        static _layout: LazyLock<Mutex<Layout>> = LazyLock::new(|| Mutex::new(Layout::_arrange()));
        _layout
            .lock()
            .as_deref_mut()
            .map(|r| ptr::from_mut(r))
            .unwrap()
    }

    #[inline]
    pub unsafe fn insert(&mut self, bundle: Bundle, binding: u32) -> &Bundle {
        self.mapping.try_insert(binding, bundle).unwrap_unchecked()
    }

    pub fn recycle(&self) {}
}
