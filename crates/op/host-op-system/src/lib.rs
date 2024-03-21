mod cpu;
mod interrupts;
mod memory;
mod os;
mod rps;
mod utils;

use wasmtime::component::Linker;

wasmtime::component::bindgen!({
    path: "../../../src/psh-sdk-wit/wit/deps/system",
    world: "imports",
});

pub struct SysCtx {
    // TODO: add more fields
}

pub fn add_to_linker<T>(
    l: &mut Linker<T>,
    f: impl (Fn(&mut T) -> &mut SysCtx) + Copy + Send + Sync + 'static,
) -> anyhow::Result<()> {
    crate::Imports::add_to_linker(l, f)
}
