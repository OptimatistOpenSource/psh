use wasmtime::component::ResourceTable;
use wasmtime_wasi::preview2::{WasiCtx, WasiView};

use host_op_perf::PerfCtx;
use host_op_system::SysCtx;

pub struct PshState {
    #[allow(dead_code)]
    pub(crate) name: String,
    pub(crate) table: ResourceTable,
    pub(crate) wasi_ctx: WasiCtx,
    pub(crate) perf_ctx: PerfCtx,
    pub(crate) sys_ctx: SysCtx,
    // TODO: add more context for modules
}

impl WasiView for PshState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}
