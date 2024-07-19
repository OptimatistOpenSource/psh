use std::path::Path;

use anyhow::Context;
use wasmtime::{
    component::{Component, Linker},
    Engine, Store,
};
use wasmtime_wasi::bindings::sync::Command;

use super::PshState;

pub struct PshEngine {
    pub(crate) engine: Engine,
    pub(crate) store: Store<PshState>,
    pub(crate) linker: Linker<PshState>,
}

impl PshEngine {
    pub fn run(&mut self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let component =
            Component::from_file(&self.engine, path).context("Failed to read component file!")?;
        let (cmd, inst) = Command::instantiate(&mut self.store, &component, &self.linker)
            .context("Failed to instantiate Wasi Command!")?;

        // FIXME: It is ugly to save `Store` and `Instance` raw pointers to `ebpf_ctx`.
        // But, to implement ebpf callback function in host side, there is no way to get current
        // `Store` and `Instance` through wasmtime's API.
        // As the lifetimes of `self.store` and `inst` are as long as the wasm component, it is safe
        // to pass them to `ebpf_ctx`.
        let store_raw_mut_ptr = &mut self.store as *mut _;
        self.store.data_mut().ebpf_ctx.set_store(store_raw_mut_ptr);
        self.store.data_mut().ebpf_ctx.set_instance(&inst);

        cmd.wasi_cli_run()
            .call_run(&mut self.store)
            .context("Failed to run component")?
            .map_err(|()| anyhow::anyhow!("Component returned an error!"))
    }
}
