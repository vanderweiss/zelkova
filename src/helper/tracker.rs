use std::sync::atomic::{AtomicU32, Ordering};

// maybe future impl for `Tracker` through `Arc<Mutex<Tracker>>, eh`
pub(crate) struct History<'t, T> {
    history: Vec<&'t T>,
}

impl<'t, T> History<'t, T> {
    fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }
    fn fetch(&self) -> u32 {
        static TRACKER: AtomicU32 = AtomicU32::new(0);
        TRACKER.fetch_add(1, Ordering::SeqCst);
    }
    fn index(&self, offset: u32) -> &T {
        self.history[offset]
    }
}

pub(crate) trait Tracker {
    fn id() -> u32 {
        static TRACKER: AtomicU32 = AtomicU32::new(0);
        TRACKER.fetch_add(1, Ordering::SeqCst);
    }
}
