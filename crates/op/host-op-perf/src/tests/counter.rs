use crate::tests::compile_component;
use crate::{PerfCtx, PerfView};
use wasmtime::component::{Component, Instance, Linker, ResourceTable};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::preview2::{command, WasiCtx, WasiCtxBuilder, WasiView};

pub struct State {
    pub perf_ctx: PerfCtx,
    pub wasi_ctx: WasiCtx,
    pub table: ResourceTable,
}

impl WasiView for State {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

impl PerfView for State {
    fn table(&self) -> &ResourceTable {
        &self.table
    }

    fn table_mut(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&self) -> &PerfCtx {
        &self.perf_ctx
    }

    fn ctx_mut(&mut self) -> &mut PerfCtx {
        &mut self.perf_ctx
    }
}

#[test]
fn test_counter() {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config).unwrap();
    let mut store = Store::new(
        &engine,
        State {
            perf_ctx: PerfCtx::new(),
            wasi_ctx: WasiCtxBuilder::new().inherit_stdout().build(),
            table: ResourceTable::new(),
        },
    );
    let mut linker: Linker<State> = Linker::new(&engine);
    crate::add_to_linker(&mut linker).unwrap();
    command::sync::add_to_linker(&mut linker).unwrap();

    let path = "../../../test_resources/profiling/test-perf-counter";
    compile_component(path);
    let wasm_path = format!("{}/target/wasm32-wasi/debug/test-perf-counter.wasm", path);
    let component = Component::from_file(&engine, wasm_path).unwrap();

    let (cmd, _): (_, Instance) =
        command::sync::Command::instantiate(&mut store, &component, &linker).unwrap();

    cmd.wasi_cli_run().call_run(&mut store).unwrap().unwrap();
}
