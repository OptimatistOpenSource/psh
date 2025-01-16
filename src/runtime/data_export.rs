// Copyright (c) 2023-2024 Optimatist Technology Co., Ltd. All rights reserved.
// DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
//
// This file is part of PSH.
//
// PSH is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// PSH is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
// the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with Performance Savior Home (PSH). If not,
// see <https://www.gnu.org/licenses/>.

use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc::{channel, SendError, Sender, TryRecvError},
        Arc,
    },
    thread::{self, JoinHandle},
};

use chrono::{TimeZone, Utc};
use profiling::data_export::common::FieldValue as WitFieldValue;
use profiling::data_export::measurement::Point;
use profiling::data_export::metric::Sample;
use prost::Message;
use rinfluxdb::line_protocol::{FieldValue, LineBuilder};
use tokio::runtime::Runtime;
use wasmtime::component::Linker;

use crate::services::{
    pb::{Data, DataType, ExportDataReq},
    rpc::RpcClient,
};

wasmtime::component::bindgen!({
    path: "psh-sdk-wit/wit/deps/data-export",
    world: "imports",
    // https://github.com/bytecodealliance/wasmtime/pull/8310
    // wasmtime have added a config in bindgen! macro to allow user specify
    // whether they want a function be able to trap(outer wasmtime::Result).
    // by default the value is false, we use true here to compatible with our
    // previous implementations.
    trappable_imports: true,
});

pub struct DataExporter {
    bytes_len: Arc<AtomicUsize>,
    bytes_watermark: usize,
    data_tx: Option<Sender<Data>>,
    exporter: JoinHandle<()>,
}

impl DataExporter {
    pub fn new(
        bytes_capacity: usize,
        bytes_watermark: usize,
        task_id: String,
        rpc_client: RpcClient,
    ) -> Self {
        // TODO: `bytes_capacity` is not used because we not have a static allocation
        // for `Data`, maybe we will remove this in the future
        let _ = bytes_capacity;
        let (data_tx, data_rx) = channel::<Data>();
        let bytes_len = Arc::new(AtomicUsize::new(0));

        let exporter = thread::spawn({
            let bytes_len = Arc::clone(&bytes_len);
            move || {
                let rt = Runtime::new().expect("Failed to init exporter runtime");
                let mut data = Vec::new();
                loop {
                    match data_rx.try_recv() {
                        Ok(o) => {
                            // No critical section, relaxed ordering is fine.
                            bytes_len.fetch_sub(o.encoded_len(), Ordering::Relaxed);
                            data.push(o);
                        }
                        Err(e) => {
                            if !data.is_empty() {
                                let merged = ExportDataReq {
                                    task_id: task_id.clone(),
                                    data: data.clone(),
                                };

                                let mut rpc_client = rpc_client.clone();
                                rt.block_on(async move {
                                    let _ = rpc_client.export_data(merged).await;
                                });
                                data.clear();
                            }
                            match e {
                                TryRecvError::Empty => thread::park(),
                                TryRecvError::Disconnected => break,
                            }
                        }
                    }
                }
            }
        });

        Self {
            bytes_len,
            bytes_watermark,
            data_tx: Some(data_tx),
            exporter,
        }
    }

    pub fn schedule(&self, data: Data) -> Result<(), SendError<Data>> {
        let encoded_len = data.encoded_len();
        // Never failes since we only take it in `Drop`.
        let data_tx = self.data_tx.as_ref().expect("unreachable");
        data_tx.send(data)?;
        // No critical section, relaxed ordering is fine.
        let prev = self.bytes_len.fetch_add(encoded_len, Ordering::Relaxed);
        if prev > self.bytes_watermark {
            self.exporter.thread().unpark();
        }
        Ok(())
    }
}

impl Drop for DataExporter {
    fn drop(&mut self) {
        // Drop sender early to prevent the exporter thread from being parked forever.
        drop(self.data_tx.take());
        self.exporter.thread().unpark();
    }
}

#[derive(Clone)]
pub struct DataExportBuf {
    bytes_watermark: usize,
    bytes_len: usize,
    reqs: VecDeque<Data>,
}

impl DataExportBuf {
    pub const fn new(bytes_capacity: usize, bytes_watermark: usize) -> Self {
        // TODO: `bytes_capacity` is not used because we not have a static allocation
        // for `ExportDataReq::data`, maybe we will remove this in the future
        let _ = bytes_capacity;
        Self {
            bytes_watermark,
            bytes_len: 0,
            reqs: VecDeque::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.reqs.len()
    }

    pub const fn watermark_reached(&self) -> bool {
        self.bytes_len >= self.bytes_watermark
    }

    pub fn push_back(&mut self, data: Data) {
        let encoded_len = data.encoded_len();
        self.reqs.push_back(data);
        self.bytes_len += encoded_len;
    }

    pub fn pop_front(&mut self) -> Option<Data> {
        let data = self.reqs.pop_front()?;
        self.bytes_len -= data.encoded_len();
        Some(data)
    }
}

fn flush_buf(ctx: &mut Ctx) {
    let mut data = Vec::with_capacity(ctx.buf.len());

    while let Some(it) = ctx.buf.pop_front() {
        data.push(it);
    }

    let merged = ExportDataReq {
        task_id: ctx.task_id.clone(),
        data,
    };
    let mut rpc_client = ctx.rpc_client.clone();
    ctx.exporter_rt
        .spawn(async move { rpc_client.export_data(merged).await });
}

fn schedule_data(ctx: &mut Ctx, data: Data) {
    ctx.buf.push_back(data);

    if ctx.buf.watermark_reached() {
        flush_buf(ctx);
    }
}

#[derive(Clone)]
pub struct Ctx {
    pub task_id: String,
    pub instance_id: String,
    pub rpc_client: RpcClient,
    pub buf: DataExportBuf,
    pub exporter_rt: Arc<Runtime>,
}

impl Drop for Ctx {
    fn drop(&mut self) {
        flush_buf(self)
    }
}

#[derive(Clone)]
pub struct DataExportCtx {
    pub ctx: Option<Ctx>,
}

impl From<WitFieldValue> for FieldValue {
    fn from(value: WitFieldValue) -> Self {
        match value {
            WitFieldValue::Float(x) => Self::Float(x),
            WitFieldValue::Int(x) => Self::Integer(x),
            WitFieldValue::Uint(x) => Self::UnsignedInteger(x),
            WitFieldValue::Text(x) => Self::String(x),
            WitFieldValue::Boolean(x) => Self::Boolean(x),
            WitFieldValue::NsTs(x) => Self::Timestamp(Utc.timestamp_nanos(x as _)),
        }
    }
}

impl profiling::data_export::common::Host for DataExportCtx {
    fn flush_buf(&mut self) -> wasmtime::Result<Result<(), String>> {
        if let Some(ctx) = &mut self.ctx {
            flush_buf(ctx);
        }
        Ok(Ok(()))
    }
}

impl profiling::data_export::file::Host for DataExportCtx {
    fn export_bytes(&mut self, bytes: Vec<u8>) -> wasmtime::Result<Result<(), String>> {
        let Some(ctx) = &mut self.ctx else {
            return Ok(Ok(()));
        };

        let data = Data {
            ty: DataType::File as _,
            bytes,
        };
        schedule_data(ctx, data);

        Ok(Ok(()))
    }
}

impl profiling::data_export::metric::Host for DataExportCtx {
    fn export_sample(&mut self, mut sample: Sample) -> wasmtime::Result<Result<(), String>> {
        let Some(ctx) = &mut self.ctx else {
            return Ok(Ok(()));
        };

        sample
            .tags
            .push(("instance_id".to_string(), ctx.instance_id.clone()));

        let bytes = {
            let mut lb = LineBuilder::new(sample.name).insert_field("value", sample.value);
            for (k, v) in sample.tags.clone() {
                lb = lb.insert_tag(k, v);
            }

            if let Some(s) = sample.ns_ts {
                let timestamp = Utc.timestamp_nanos(s as _);
                lb = lb.set_timestamp(timestamp)
            };

            lb.build().to_string().into_bytes()
        };
        let data = Data {
            ty: DataType::LineProtocol as _,
            bytes,
        };
        schedule_data(ctx, data);

        Ok(Ok(()))
    }
}

impl profiling::data_export::measurement::Host for DataExportCtx {
    fn export_point(&mut self, mut point: Point) -> wasmtime::Result<Result<(), String>> {
        let Some(ctx) = &mut self.ctx else {
            return Ok(Ok(()));
        };

        point
            .tags
            .push(("instance_id".to_string(), ctx.instance_id.clone()));

        let bytes = {
            let mut lb = LineBuilder::new(point.name);
            for (k, v) in point.tags.clone() {
                lb = lb.insert_tag(k, v);
            }
            for (k, v) in point.fields {
                lb = lb.insert_field(k, v);
            }

            if let Some(s) = point.ns_ts {
                let timestamp = Utc.timestamp_nanos(s as _);
                lb = lb.set_timestamp(timestamp)
            };

            lb.build().to_string().into_bytes()
        };
        let data = Data {
            ty: DataType::LineProtocol as _,
            bytes,
        };
        schedule_data(ctx, data);

        Ok(Ok(()))
    }
}

pub fn add_to_linker<T>(
    l: &mut Linker<T>,
    f: impl (Fn(&mut T) -> &mut DataExportCtx) + Copy + Send + Sync + 'static,
) -> anyhow::Result<()> {
    Imports::add_to_linker(l, f)
}
