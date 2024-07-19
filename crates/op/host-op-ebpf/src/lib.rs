use std::{
    collections::HashMap,
    ffi::CString,
    fs::File,
    os::fd::{AsFd, AsRawFd},
    ptr,
    sync::Mutex,
    time::Duration,
};

use errno::errno;
use lazy_static::lazy_static;
use libbpf_rs::{libbpf_sys, AsRawLibbpf, Link, Map, Object, ObjectBuilder, PerfBufferBuilder};
use libc::if_nametoindex;
use profiling::ebpf::ebpf::{self, Key, MapFlags, Value};
use state::BpfCtx;
use wasmtime::{component::Linker, AsContextMut};

pub mod state;

pub struct ObjectWrapper(Object);
/// Here we explicitly unsafe impl Send trait for ObjectWrapper is because
/// there is a !Send `NonNull<T>` type field inside `libpbf_rs::Object`. But wasmtime
/// ResourceTable.push() method asks for Send + 'static types.
unsafe impl Send for ObjectWrapper {}

/// N.B. It's not allowed to put lifetime reference within an object into `ResourceTable`
/// We have to make compromise by using global `HashMap` to save resource handle.
///
/// https://bytecodealliance.zulipchat.com/#narrow/stream/217126-wasmtime/topic/ResourceTable.20with.20lifetime.20field.2E/near/449884467
///
/// An `Invalid argument` error maybe returned to the wasm component if it uses `Map` and `PerfBuffer`
/// resources after dropping `Bpf`.

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct MapKey(String);

pub struct MapWrapper {
    /// save the raw pointer that points to a `libbpf_rs::Map` inside `libbpf_rs::Object`
    /// eliminated original lifetime of `libbpf_rs::Map` reference.
    map: *const Map,
}

/// Explicitly unsafe impl `Send` and `Sync` traits for `MapWrapper` as
/// we will put `MapWrapper` to a `HashMap` that auto implemented `Send` and `Sync` traits for `K` and `V`.
///
/// https://doc.rust-lang.org/std/collections/struct.HashMap.html#impl-Send-for-HashMap%3CK,+V,+S%3E
///
/// It is safe to do this because our `MapWrapper` won't share with other threads or be accessed concurrently.
unsafe impl Send for MapWrapper {}
unsafe impl Sync for MapWrapper {}

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct PerfBufferKey(u64);

struct PerfBufferWrapper<'a>(libbpf_rs::PerfBuffer<'a>);

// ditto
unsafe impl<'a> Sync for PerfBufferWrapper<'a> {}

pub struct MapKeyIter {
    fd: i32,
    prev: Option<Vec<u8>>,
    next: Vec<u8>,
}

impl MapKeyIter {
    fn new(fd: i32, key_size: u32) -> Self {
        Self {
            fd,
            prev: None,
            next: vec![0; key_size as usize],
        }
    }
}

impl Iterator for MapKeyIter {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let prev = self.prev.as_ref().map_or(ptr::null(), |p| p.as_ptr());
        let ret = unsafe {
            libbpf_sys::bpf_map_get_next_key(self.fd, prev as _, self.next.as_mut_ptr() as _)
        };
        if ret != 0 {
            None
        } else {
            self.prev = Some(self.next.clone());
            Some(self.next.clone())
        }
    }
}

pub struct Bpf {
    pub object: ObjectWrapper,
    /// Files that are opened by bpf programs.
    pub opened_files: Vec<File>,
    /// Links that are returned after attach.
    pub opened_links: Vec<Link>,
}

lazy_static! {
    static ref BPF_MAP_HASHMAP: Mutex<HashMap<MapKey, MapWrapper>> = Mutex::new(HashMap::new());
    static ref BPF_PERF_BUFFER_HASHMAP: Mutex<HashMap<PerfBufferKey, PerfBufferWrapper<'static>>> =
        Mutex::new(HashMap::new());
}

wasmtime::component::bindgen!({
    path: "../../../psh-sdk-wit/wit/deps/ebpf",
    world: "imports",
    with: {
        "profiling:ebpf/ebpf/bpf" : Bpf,
        "profiling:ebpf/ebpf/map" : MapKey,
        "profiling:ebpf/ebpf/map-key-iter" : MapKeyIter,
        "profiling:ebpf/ebpf/perf-buffer": PerfBufferKey,
        // TODO (Chengdong Li), Add more resources if we have. `ring-buffer`?
    },
    // https://github.com/bytecodealliance/wasmtime/pull/8310
    // wasmtime have added a config in bindgen! macro to allow user specify
    // whether they want a function be able to trap(outer wasmtime::Result).
    // by default the value is false, we use true here to compatible with our
    // previous implementations.
    trappable_imports: true,
});

impl<State> ebpf::HostBpf for BpfCtx<State> {
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
                    object: ObjectWrapper(object),
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
        let bpf: &mut Bpf = self.table.get_mut(&self_)?;
        let object = &mut bpf.object.0;

        match object.prog_mut(&name) {
            Some(prog) => {
                if !target.is_empty() {
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
        Result<wasmtime::component::Resource<MapKey>, wasmtime::component::__internal::String>,
        wasmtime::Error,
    > {
        let table = &mut self.table;
        let bpf: &mut Bpf = table.get_mut(&self_)?;
        let object = &bpf.object.0;
        match object.map(&name) {
            Some(map) => {
                let key = MapKey(name);
                let value = MapWrapper {
                    map: map as *const Map,
                };
                BPF_MAP_HASHMAP.lock().unwrap().insert(key.clone(), value);
                Ok(Ok(table.push(key)?))
            }
            None => Ok(Err(format!("Invalid map name: {}", name).to_string())),
        }
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<Bpf>) -> wasmtime::Result<()> {
        // clean global `HashMap`s to avoid use after free.
        BPF_PERF_BUFFER_HASHMAP.lock().unwrap().clear();
        BPF_MAP_HASHMAP.lock().unwrap().clear();
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

impl<State> ebpf::HostMap for BpfCtx<State> {
    /// return iterable map keys.
    fn keys(
        &mut self,
        self_: wasmtime::component::Resource<MapKey>,
    ) -> Result<Result<wasmtime::component::Resource<MapKeyIter>, String>, wasmtime::Error> {
        let map_wrapper: &MapKey = self.table.get(&self_)?;

        match BPF_MAP_HASHMAP.lock().unwrap().get(map_wrapper) {
            Some(wrapper) => {
                let map = unsafe { &*wrapper.map };
                let keys = MapKeyIter::new(map.as_fd().as_raw_fd(), map.key_size());
                Ok(Ok(self.table.push(keys)?))
            }
            None => Ok(Err("Invalid argument".to_string())),
        }
    }

    /// The BPF_MAP_LOOKUP_ELEM command looks up an element with a
    /// given key in the map.
    /// If an element is found, the value is returned.
    /// An error message is returned if no element is found.
    fn lookup_elem(
        &mut self,
        self_: wasmtime::component::Resource<MapKey>,
        key: Key,
        flags: MapFlags,
    ) -> Result<Result<Option<Value>, wasmtime::component::__internal::String>, wasmtime::Error>
    {
        let map_wrapper: &MapKey = self.table.get(&self_)?;
        match BPF_MAP_HASHMAP.lock().unwrap().get(map_wrapper) {
            Some(wrapper) => {
                let flags = match libbpf_rs::MapFlags::try_from(flags) {
                    Ok(flags) => flags,
                    Err(e) => return Ok(Err(e.to_string())),
                };
                let map = unsafe { &*wrapper.map };
                Ok(map.lookup(&key, flags).map_err(|e| e.to_string()))
            }
            None => Ok(Err("Invalid argument".to_string())),
        }
    }

    /// The BPF_MAP_UPDATE_ELEM command creates or updates an
    /// element with a given key/value in the map.
    ///
    /// If the element is not found, an error message is returned.
    fn update_elem(
        &mut self,
        self_: wasmtime::component::Resource<MapKey>,
        key: Key,
        value: Value,
        flags: MapFlags,
    ) -> Result<Result<(), wasmtime::component::__internal::String>, wasmtime::Error> {
        let map_wrapper: &MapKey = self.table.get(&self_)?;

        match BPF_MAP_HASHMAP.lock().unwrap().get(map_wrapper) {
            Some(wrapper) => {
                let flags = match libbpf_rs::MapFlags::try_from(flags) {
                    Ok(flags) => flags,
                    Err(e) => return Ok(Err(e.to_string())),
                };
                let map = unsafe { &*wrapper.map };
                Ok(map.update(&key, &value, flags).map_err(|e| e.to_string()))
            }
            None => Ok(Err("Invalid argument".to_string())),
        }
    }

    /// The BPF_MAP_DELETE_ELEM command deletes the element whose
    /// key is key from the map.
    ///
    /// If the element is not found, an error message is returned.
    fn delete_elem(
        &mut self,
        self_: wasmtime::component::Resource<MapKey>,
        key: Key,
    ) -> Result<Result<(), wasmtime::component::__internal::String>, wasmtime::Error> {
        let map_wrapper: &MapKey = self.table.get(&self_)?;
        match BPF_MAP_HASHMAP.lock().unwrap().get(map_wrapper) {
            Some(wrapper) => {
                let map = unsafe { &*wrapper.map };
                Ok(map.delete(&key).map_err(|e| e.to_string()))
            }
            None => Ok(Err("Invalid argument".to_string())),
        }
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<MapKey>) -> wasmtime::Result<()> {
        let map_wrapper: &MapKey = self.table.get(&rep)?;
        BPF_MAP_HASHMAP.lock().unwrap().remove(map_wrapper);
        self.table.delete(rep)?;
        Ok(())
    }
}

impl<State> ebpf::HostMapKeyIter for BpfCtx<State> {
    fn next(
        &mut self,
        self_: wasmtime::component::Resource<MapKeyIter>,
    ) -> Result<Result<Option<Vec<u8>>, ()>, wasmtime::Error> {
        let map_key_iter: &mut MapKeyIter = self.table.get_mut(&self_)?;
        Ok(Ok(map_key_iter.next()))
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<MapKeyIter>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}

impl<State: 'static> ebpf::HostPerfBuffer for BpfCtx<State> {
    fn new(
        &mut self,
        map_key: wasmtime::component::Resource<MapKey>,
        pages: u64,
        sample_cb_name: std::string::String,
        lost_cb_name: std::string::String,
    ) -> Result<Result<wasmtime::component::Resource<PerfBufferKey>, String>, wasmtime::Error> {
        let map_wrapper: &MapKey = self.table.get(&map_key)?;
        let map = match BPF_MAP_HASHMAP.lock().unwrap().get(map_wrapper) {
            Some(wrapper) => unsafe { &*wrapper.map },
            None => return Ok(Err("Invalid argument (map)".to_string())),
        };

        static INTERNAL_ERROR: &str =
            "Internal error: calling this method before context initialization";
        if self.instance.is_null() || self.store.is_null() {
            return Ok(Err(INTERNAL_ERROR.to_string()));
        }

        let instance = unsafe { &*self.instance };

        // N.B. unsafely duplicate store mutable reference here, as we can
        // make sure that `sample_cb` and `lost_cb` will not be called concurrently.
        let (store_dup, store_dup2) = unsafe {
            let s = self.store;
            (&mut *s, &mut *s)
        };

        let sample_cb = match instance.get_func(store_dup2.as_context_mut(), &sample_cb_name) {
            Some(func) => {
                let typed_func = match func.typed::<(i32, &[u8]), ()>(store_dup2.as_context_mut()) {
                    Ok(typed_func) => typed_func,
                    Err(_) => return Ok(Err(format!("{sample_cb_name} signature mismatch"))),
                };

                move |cpu: i32, data: &[u8]| {
                    let _ = typed_func.call(store_dup2.as_context_mut(), (cpu, data));
                }
            }
            None => return Ok(Err(format!("{sample_cb_name} is not found"))),
        };

        let perf_buffer = if !lost_cb_name.is_empty() {
            let lost_cb = match instance.get_func(store_dup.as_context_mut(), &lost_cb_name) {
                Some(func) => {
                    let lost_func_typed =
                        match func.typed::<(i32, u64), ()>(store_dup.as_context_mut()) {
                            Ok(typed_func) => typed_func,
                            Err(_) => return Ok(Err(format!("{lost_cb_name} signature mismatch"))),
                        };
                    lost_func_typed
                }
                None => return Ok(Err(format!("{lost_cb_name} is not found"))),
            };

            PerfBufferBuilder::new(map)
                .sample_cb(sample_cb)
                .lost_cb(move |cpu: i32, count: u64| {
                    let _ = lost_cb.call(store_dup.as_context_mut(), (cpu, count));
                })
                .pages(pages as usize)
                .build()?
        } else {
            PerfBufferBuilder::new(map)
                .pages(pages as usize)
                .sample_cb(sample_cb)
                .lost_cb(move |_cpu: i32, _count: u64| {})
                .build()?
        };

        let key = PerfBufferKey(perf_buffer.as_libbpf_object().as_ptr() as u64);

        BPF_PERF_BUFFER_HASHMAP
            .lock()
            .unwrap()
            .insert(key.clone(), PerfBufferWrapper(perf_buffer));

        Ok(Ok(self.table.push(key)?))
    }

    fn epoll_fd(
        &mut self,
        self_: wasmtime::component::Resource<PerfBufferKey>,
    ) -> wasmtime::Result<i32> {
        let perf_buffer_key: &PerfBufferKey = self.table.get(&self_)?;

        match BPF_PERF_BUFFER_HASHMAP.lock().unwrap().get(perf_buffer_key) {
            Some(perf_buffer) => Ok(perf_buffer.0.epoll_fd()),
            None => Ok(-1),
        }
    }

    fn poll(
        &mut self,
        self_: wasmtime::component::Resource<PerfBufferKey>,
        timeout_ms: u64,
    ) -> wasmtime::Result<Result<(), String>> {
        let timeout = Duration::from_millis(timeout_ms);
        let perf_buffer_key: &PerfBufferKey = self.table.get(&self_)?;

        match BPF_PERF_BUFFER_HASHMAP.lock().unwrap().get(perf_buffer_key) {
            Some(perf_buffer) => Ok(perf_buffer.0.poll(timeout).map_err(|e| e.to_string())),
            None => Ok(Err("Invalid Param".to_string())),
        }
    }

    fn consume(
        &mut self,
        self_: wasmtime::component::Resource<PerfBufferKey>,
    ) -> wasmtime::Result<Result<(), String>> {
        let perf_buffer_key: &PerfBufferKey = self.table.get(&self_)?;

        match BPF_PERF_BUFFER_HASHMAP.lock().unwrap().get(perf_buffer_key) {
            Some(perf_buffer) => Ok(perf_buffer.0.consume().map_err(|e| e.to_string())),
            None => Ok(Err("Invalid Param".to_string())),
        }
    }

    fn consume_buffer(
        &mut self,
        self_: wasmtime::component::Resource<PerfBufferKey>,
        buf_idx: u64,
    ) -> wasmtime::Result<Result<(), String>> {
        let perf_buffer_key: &PerfBufferKey = self.table.get(&self_)?;

        match BPF_PERF_BUFFER_HASHMAP.lock().unwrap().get(perf_buffer_key) {
            Some(perf_buffer) => Ok(perf_buffer
                .0
                .consume_buffer(buf_idx as usize)
                .map_err(|e| e.to_string())),
            None => Ok(Err("Invalid Param".to_string())),
        }
    }

    fn buffer_cnt(
        &mut self,
        self_: wasmtime::component::Resource<PerfBufferKey>,
    ) -> wasmtime::Result<Result<u64, String>> {
        let perf_buffer_key: &PerfBufferKey = self.table.get(&self_)?;

        match BPF_PERF_BUFFER_HASHMAP.lock().unwrap().get(perf_buffer_key) {
            Some(perf_buffer) => Ok(Ok(perf_buffer.0.buffer_cnt() as u64)),
            None => Ok(Err("Invalid Param".to_string())),
        }
    }

    fn buffer_fd(
        &mut self,
        self_: wasmtime::component::Resource<PerfBufferKey>,
        buf_idx: u64,
    ) -> wasmtime::Result<Result<i32, String>> {
        let perf_buffer_key: &PerfBufferKey = self.table.get(&self_)?;

        match BPF_PERF_BUFFER_HASHMAP.lock().unwrap().get(perf_buffer_key) {
            Some(perf_buffer) => {
                let res = perf_buffer
                    .0
                    .buffer_fd(buf_idx as usize)
                    .map_err(|e| e.to_string());
                Ok(res)
            }
            None => Ok(Err("Invalid Param".to_string())),
        }
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<PerfBufferKey>) -> wasmtime::Result<()> {
        let perf_buffer_key: &PerfBufferKey = self.table.get(&rep)?;
        BPF_PERF_BUFFER_HASHMAP
            .lock()
            .unwrap()
            .remove(perf_buffer_key);
        self.table.delete(rep)?;
        Ok(())
    }
}

impl<State: 'static> ebpf::Host for BpfCtx<State> {}

pub fn add_to_linker<T: 'static>(
    l: &mut Linker<T>,
    f: impl (Fn(&mut T) -> &mut BpfCtx<T>) + Copy + Send + Sync + 'static,
) -> anyhow::Result<()> {
    crate::Imports::add_to_linker(l, f)
}
