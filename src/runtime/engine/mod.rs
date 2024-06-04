use std::path::Path;

use anyhow::Context;
use wasmtime::{
    component::{Component, Linker},
    Engine, Store,
};
use wasmtime_wasi::bindings::Command;

use super::PshState;

pub struct PshEngine {
    pub(crate) engine: Engine,
    pub(crate) store: Store<PshState>,
    pub(crate) linker: Linker<PshState>,
}

impl PshEngine {
    pub async fn run(&mut self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let component =
            Component::from_file(&self.engine, path).context("Failed to read component file!")?;
        let (cmd, _inst) = Command::instantiate_async(&mut self.store, &component, &self.linker)
            .await
            .context("Failed to instantiate Wasi Command!")?;
        cmd.wasi_cli_run()
            .call_run(&mut self.store)
            .await
            .context("Failed to run component")?
            .map_err(|()| anyhow::anyhow!("Component returned an error!"))
    }
}
