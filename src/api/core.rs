use {
    std::{
        any,
        collections::HashMap,
        ptr,
        sync::{LazyLock, Mutex},
    },
    wgpu,
};

use crate::internals::{Buffer, BufferType, Component};

/// Allocation type.
pub(crate) enum VMemory {
    Runtime,
    Static,
}

/// State to keep track of bundle during the compute process.
pub(crate) enum VState {
    Allocated,
    Binded,
    Prepared,
}

/// Handler for wgpu buffer representations. A thin wrapper that aids with codegen and operations performed on them.
/// Shouldn't be directly intantiated.
pub(crate) struct Bundle {
    pub buffer: Buffer,
    pub memory: VMemory,
    pub state: VState,

    #[doc(hidden)]
    pub _alias: &'static str,
    #[doc(hidden)]
    pub _binding: u32,
    #[doc(hidden)]
    pub _group: u32,
}

impl Bundle {
    pub fn bind<C: Component>(_src: &[C], binding: u32) -> Result<&Self, wgpu::Error> {
        let layout = Layout::arrange();

        let bundle = unsafe {
            (*layout).insert(
                Self {
                    buffer: Buffer::bind::<_>(BufferType::Factor, Some(_src))?,
                    memory: VMemory::Static,
                    state: VState::Binded,

                    _alias: any::type_name::<C>(),
                    _binding: binding,
                    _group: 0,
                },
                binding,
            )
        };

        dbg!(bundle.tag());

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

use crate::codegen::Element;
