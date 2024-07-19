use wasmtime::{component::Instance, Store};
use wasmtime_wasi::ResourceTable;

/// The application state
pub struct BpfCtx<State> {
    pub table: ResourceTable,
    pub store: *mut Store<State>,
    pub instance: *const Instance,
}

unsafe impl<State> Send for BpfCtx<State> {}

impl<State> Default for BpfCtx<State> {
    fn default() -> Self {
        Self::new()
    }
}

impl<State> BpfCtx<State> {
    /// Create an AppState
    pub fn new() -> Self {
        Self {
            table: ResourceTable::new(),
            store: std::ptr::null_mut(),
            instance: std::ptr::null_mut(),
        }
    }

    pub fn set_store(&mut self, store: *mut Store<State>) {
        self.store = store
    }

    pub fn set_instance(&mut self, instance: *const Instance) {
        self.instance = instance
    }
}
