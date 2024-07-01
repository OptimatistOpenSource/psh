use std::{
    borrow::{Borrow, BorrowMut},
    cell::{Cell, RefCell},
    ffi::CString,
    fs::File,
    os::fd::{AsFd, AsRawFd},
    rc::Rc,
    sync::{Arc, Mutex},
};

use errno::errno;
use libbpf_rs::{Link, Map, Object, ObjectBuilder, PerfBufferBuilder};
use libc::{if_nameindex, if_nametoindex};
use profiling::ebpf::ebpf::{self, Key, MapFlags, Value};
use state::BpfCtx;
use wasmtime::{component::Linker, Func, InstancePre, Table};

//pub mod attach;
//pub mod close;
//pub mod fd_by_name;
//pub mod load;
//pub mod map_operate;
pub mod poll;
mod state;
//mod utils;
//pub mod wrapper_poll;

// pub(crate) const EINVAL: i32 = 22;
// pub(crate) const ENOENT: i32 = 2;

// #[macro_export]
// macro_rules! ensure_program_mut_by_state {
//     ($state: expr, $program: expr) => {
//         match $state.object_map.get_mut(&$program) {
//             Some(v) => v,
//             None => {
//                 log::debug!("Invalid program: {}", $program);
//                 return -1;
//             }
//         }
//     };
// }

// #[macro_export]
// macro_rules! ensure_program_by_state {
//     ($state: expr, $program: expr) => {
//         match $state.object_map.get(&$program) {
//             Some(v) => v,
//             None => {
//                 log::debug!("Invalid program: {}", $program);
//                 return -1;
//             }
//         }
//     };
// }

// #[macro_export]
// macro_rules! ensure_program_mut_by_caller {
//     ($caller: expr, $program: expr) => {{
//         use $crate::ensure_program_mut_by_state;
//         ensure_program_mut_by_state!($caller.data_mut(), $program)
//     }};
// }

// #[macro_export]
// macro_rules! ensure_program_by_caller {
//     ($caller: expr, $program: expr) => {{
//         use $crate::ensure_program_by_state;
//         ensure_program_by_state!($caller.data_mut(), $program)
//     }};
// }

// #[macro_export]
// macro_rules! ensure_c_str {
//     ($caller: expr, $var_name: expr) => {{
//         use $crate::utils::CallerUtils;
//         match $caller.read_zero_terminated_str($var_name as usize) {
//             Ok(v) => v.to_string(),
//             Err(err) => {
//                 log::debug!("Failed to read `{}`: {}", stringify!($var_name), err);
//                 return -1;
//             }
//         }
//     }};
// }
// /// The pointer type in 32bit wasm
// pub type WasmPointer = u32;
// /// The handle to a bpf object
// pub type BpfObjectType = u64;
// /// The string type in wasm, is also a pointer
// pub type WasmString = u32;

// #[macro_export]
// macro_rules! ensure_enough_memory {
//     ($caller: expr, $pointer:expr, $size: expr, $return_val: expr) => {{
//         use $crate::utils::CallerUtils;
//         let mut buf = vec![0u8];
//         match $caller
//             .get_memory()
//             .expect("Expected exported memory!")
//             .read(
//                 &mut $caller,
//                 $pointer as usize + $size as usize - 1,
//                 &mut buf,
//             ) {
//             Ok(_) => {}
//             Err(err) => {
//                 debug!("Invalid pointer for {}: {}", stringify!($pointer), err);
//                 return $return_val;
//             }
//         }
//     }};
// }

struct WrapperObject(Object);
/// Here we explicitly unsafe impl Send trait for ObjectWrapper is because
/// there is a !Send `NonNull<T>` type field inside `libpbf_rs::Object`. But wasmtime
/// ResourceTable.push() method asks for Send + 'static types.
unsafe impl Send for WrapperObject {}

pub struct WrapperMap(libbpf_rs::Map);
/// ditto
unsafe impl Send for WrapperMap {}

pub struct Bpf {
    //pub object: Arc<Mutex<WrapperObject>>,
    pub object: WrapperObject,
    /// The poller; It will be set when the first time to call the sampling function
    //pub poll_buffer: Option<PollBuffer>,
    /// Files that are opened by bpf programs.
    pub opened_files: Vec<File>,
    /// Links that are returned after attach.
    pub opened_links: Vec<Link>,
}

wasmtime::component::bindgen!({
    path: "../../../psh-sdk-wit/wit/deps/ebpf",
    world: "imports",
    with: {
        "profiling:ebpf/ebpf/bpf" : Bpf,
        "profiling:ebpf/ebpf/map" : WrapperMap,
        "profiling:ebpf/ebpf/map-key-iter" : libbpf_rs::MapKeyIter,
        "profiling:ebpf/ebpf/perf-buffer": libbpf_rs::PerfBuffer,
    },
    // https://github.com/bytecodealliance/wasmtime/pull/8310
    // wasmtime have added a config in bindgen! macro to allow user specify
    // whether they want a function be able to trap(outer wasmtime::Result).
    // by default the value is false, we use true here to compatible with our
    // previous implementations.
    trappable_imports: true,
});

impl<'a, 'store, State> ebpf::HostBpf for BpfCtx<'a, 'store, State> {
    fn load_object(
        &mut self,
        obj_buf: wasmtime::component::__internal::Vec<u8>,
    ) -> Result<
        Result<wasmtime::component::Resource<Bpf>, wasmtime::component::__internal::String>,
        wasmtime::Error,
    > {
        match ObjectBuilder::default().open_memory(&obj_buf) {
            Ok(oo) => match oo.load() {
                Ok(object) => Ok(Ok(self.table.push(Bpf {
                    object: WrapperObject(object),
                    //poll_buffer: None,
                    opened_files: Vec::new(),
                    opened_links: Vec::new(),
                })?)),
                Err(err) => Ok(Err(err.to_string())),
            },
            Err(err) => Ok(Err(err.to_string())),
        }
    }

    fn attach_program(
        &mut self,
        self_: wasmtime::component::Resource<Bpf>,
        name: wasmtime::component::__internal::String,
        target: wasmtime::component::__internal::String,
        pfd: i32,
    ) -> Result<Result<(), wasmtime::component::__internal::String>, wasmtime::Error> {
        let bpf: &Bpf = self.table.get(&self_)?;
        let mut object = bpf.object.0;

        match object.prog(&name) {
            Some(prog) => {
                if target.len() > 0 {
                    match prog.section() {
                        "sockops" => match std::fs::OpenOptions::new().read(true).open(target) {
                            Ok(cgroup_file) => {
                                let fd = cgroup_file.as_raw_fd();
                                bpf.opened_files.push(cgroup_file);
                                let link = match prog.attach_cgroup(fd) {
                                    Ok(link) => link,
                                    Err(e) => return Ok(Err(e.to_string())),
                                };
                                bpf.opened_links.push(link)
                            }
                            Err(err) => return Ok(Err(err.to_string())),
                        },
                        "xdp" => {
                            let target_c_string = match CString::new(target.as_bytes()) {
                                Ok(cstr) => cstr,
                                Err(e) => return Ok(Err(e.to_string())),
                            };
                            let ifidx = unsafe { if_nametoindex(target_c_string.as_ptr()) };
                            if ifidx == 0 {
                                return Ok(Err(errno().to_string()));
                            }

                            let link = match prog.attach_xdp(ifidx as i32) {
                                Ok(link) => link,
                                Err(e) => return Ok(Err(e.to_string())),
                            };

                            bpf.opened_links.push(link)
                        }
                        "perf_event" => {
                            let link = match prog.attach_perf_event(pfd) {
                                Ok(link) => link,
                                Err(e) => return Ok(Err(e.to_string())),
                            };
                            bpf.opened_links.push(link)
                        }
                        s => return Ok(Err(format!("Unsupported attach type: {}", s).to_string())),
                    }
                }
            }
            None => return Ok(Err(format!("No program named `{}`", name).to_string())),
        }

        Ok(Ok(()))
    }

    fn get_map_by_name(
        &mut self,
        self_: wasmtime::component::Resource<Bpf>,
        name: wasmtime::component::__internal::String,
    ) -> Result<
        Result<wasmtime::component::Resource<WrapperMap>, wasmtime::component::__internal::String>,
        wasmtime::Error,
    > {
        // let module = self.instance.unwrap().get_module(self.store, name).unwrap();

        // let table = module.get_export("__indirect_function_table").unwrap();
        // let table = table.table().unwrap();

        // let indirect_func_table: Table =  table.into();
        // let func = indirect_func_table.get(&mut store, index).unwrap().as_func().unwrap().unwrap();



        let bpf: &Bpf = self.table.get(&self_)?;
        let mut object = bpf.object.0;
        match object.map(&name) {
            Some(map) => Ok(Ok(self.table.push(WrapperMap(*map))?)),
            None => Ok(Err(format!("Invalid map name: {}", name).to_string())),
        }
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<Bpf>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}

impl TryFrom<MapFlags> for libbpf_rs::MapFlags {
    type Error = &'static str;

    fn try_from(value: MapFlags) -> Result<Self, Self::Error> {
        if value.contains(MapFlags::BPF_ANY) {
            Ok(libbpf_rs::MapFlags::ANY)
        } else if value.contains(MapFlags::BPF_EXIST) {
            Ok(libbpf_rs::MapFlags::EXIST)
        } else if value.contains(MapFlags::BPF_NOEXIST) {
            Ok(libbpf_rs::MapFlags::NO_EXIST)
        } else if value.contains(MapFlags::BPF_F_LOCK) {
            Ok(libbpf_rs::MapFlags::LOCK)
        } else {
            Err("Invalid flags")
        }
    }
}

impl<'a, 'store, State> ebpf::HostMap for BpfCtx<'a, 'store, State> {
    /// return iterable map keys.
    fn keys(
        &mut self,
        self_: wasmtime::component::Resource<WrapperMap>,
    ) -> Result<wasmtime::component::Resource<libbpf_rs::MapKeyIter<'_>>, wasmtime::Error> {
        let map: &WrapperMap = self.table.get(&self_)?;
        Ok(self.table.push(map.0.keys())?)
    }

    /// The BPF_MAP_LOOKUP_ELEM command looks up an element with a
    /// given key in the map.
    /// If an element is found, the value is returned.
    /// An error message is returned if no element is found.
    fn lookup_elem(
        &mut self,
        self_: wasmtime::component::Resource<WrapperMap>,
        key: Key,
        flags: MapFlags,
    ) -> Result<Result<Option<Value>, wasmtime::component::__internal::String>, wasmtime::Error>
    {
        let map: &WrapperMap = self.table.get(&self_)?;

        let flags = match libbpf_rs::MapFlags::try_from(flags) {
            Ok(flags) => flags,
            Err(e) => return Ok(Err(e.to_string())),
        };

        Ok(map.0.lookup(&key, flags).map_err(|e| e.to_string()))
    }

    /// The BPF_MAP_UPDATE_ELEM command creates or updates an
    /// element with a given key/value in the map.
    ///
    /// If the element is not found, an error message is returned.
    fn update_elem(
        &mut self,
        self_: wasmtime::component::Resource<WrapperMap>,
        key: Key,
        value: Value,
        flags: MapFlags,
    ) -> Result<Result<(), wasmtime::component::__internal::String>, wasmtime::Error> {
        let map: &WrapperMap = self.table.get(&self_)?;

        let flags = match libbpf_rs::MapFlags::try_from(flags) {
            Ok(flags) => flags,
            Err(e) => return Ok(Err(e.to_string())),
        };

        Ok(map.0.update(&key, &value, flags).map_err(|e| e.to_string()))
    }

    /// The BPF_MAP_DELETE_ELEM command deletes the element whose
    /// key is key from the map.
    ///
    /// If the element is not found, an error message is returned.
    fn delete_elem(
        &mut self,
        self_: wasmtime::component::Resource<WrapperMap>,
        key: Key,
    ) -> Result<Result<(), wasmtime::component::__internal::String>, wasmtime::Error> {
        let map: &WrapperMap = self.table.get(&self_)?;
        Ok(map.0.delete(&key).map_err(|e| e.to_string()))
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<WrapperMap>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}

impl<'a, 'store, State> ebpf::HostMapKeyIter for BpfCtx<'a, 'store, State> {
    fn next(
        &mut self,
        self_: wasmtime::component::Resource<libbpf_rs::MapKeyIter>,
    ) -> Result<Result<Option<Vec<u8>>, ()>, wasmtime::Error> {
        let map_key_iter: &libbpf_rs::MapKeyIter = self.table.get(&self_)?;
        Ok(Ok(map_key_iter.next()))
    }

    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<libbpf_rs::MapKeyIter>,
    ) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}

impl<'a, 'store, State> ebpf::HostPerfBuffer for BpfCtx<'a, 'store, State> {
    fn new(
        &mut self,
        map: wasmtime::component::Resource<WrapperMap>,
        pages: u64,
        sample_cb_name: std::string::String,
        lost_cb_name: std::string::String,
    ) -> Result<Result<wasmtime::component::Resource<libbpf_rs::PerfBuffer>, ()>, wasmtime::Error>
    {
        let map: &WrapperMap = self.table.get(&map)?;

        let instance = match self.instance {
            Some(instance) => instance,
            None => return Ok(Err(()))
        };
        let store = match self.store {
            Some(store) => store,
            None => return Ok(Err(()))
        };
        
        let sample_cb = match instance.get_func(store, &sample_cb_name) {
            Some(func) => {
                let typed_func = match func.typed::<(i32, &[u8]), ()>(store) {
                   Ok(typed_func) => typed_func,
                   Err(_) => return Ok(Err(()))
                };

                let store2 = store
                
                let callback = |cpu: i32, data: &[u8]| {
                    typed_func.call(store, (cpu, data));
                };
                callback
            },
            None => return Ok(Err(()))
        };

        let perf_buffer = if lost_cb_name.len() != 0 {
            let lost_cb = match instance.get_func(store, &lost_cb_name) {
                Some(func) => {
                    let lost_func_typed = match func.typed::<(i32, u64), ()>(store) {
                        Ok(typed_func) => typed_func,
                        Err(_) => return Ok(Err(()))
                    };
                    lost_func_typed
                },
                None => return Ok(Err(()))
            };
            PerfBufferBuilder::new(&map.0)
                .pages(pages as usize)
                .sample_cb(sample_cb)
                .lost_cb(move |cpu: i32, count: u64| {lost_cb.call(store, (cpu, count));})
                .build()?
        } else {
            PerfBufferBuilder::new(&map.0)
                .pages(pages as usize)
                .sample_cb(sample_cb)
                .lost_cb(move |cpu: i32, count: u64| {})
                .build()?
        };
        
        Ok(Ok(self.table.push(perf_buffer)?))
    }
}

impl<'a, 'store, State> ebpf::Host for BpfCtx<'a, 'store, State> {}

pub fn add_to_linker<'a, 'store, T: 'store>(
    l: &mut Linker<T>,
    f: impl (Fn(&mut T) -> &mut BpfCtx<'a, 'store, T>) + Copy + Send + Sync + 'static,
) -> anyhow::Result<()> {
    crate::Imports::add_to_linker(l, f)
}
