use wasmtime::component::ResourceTable;
use wasmtime_wasi::{WasiCtx, WasiView};

use host_op_ebpf::state::BpfCtx;
use host_op_perf::PerfCtx;
use host_op_system::SysCtx;

pub struct PshState {
    #[allow(dead_code)]
    pub(crate) name: String,
    pub(crate) table: ResourceTable,
    pub(crate) wasi_ctx: WasiCtx,
    pub(crate) perf_ctx: PerfCtx,
    pub(crate) sys_ctx: SysCtx,
    pub(crate) ebpf_ctx: BpfCtx<PshState>,
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
