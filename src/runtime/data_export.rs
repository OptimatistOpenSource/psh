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
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
};

use chrono::{TimeZone, Utc};
use crossbeam::queue::SegQueue;
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
    data_queue: Arc<SegQueue<Option<Data>>>,
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
        let data_queue = Arc::new(SegQueue::<Option<Data>>::new());
        let bytes_len = Arc::new(AtomicUsize::new(0));

        let exporter = thread::spawn({
            let data_queue = Arc::clone(&data_queue);
            let bytes_len = Arc::clone(&bytes_len);
            move || {
                let rt = Runtime::new().expect("Failed to init exporter runtime");
                let mut data = Vec::new();
                loop {
                    match data_queue.pop() {
                        Some(Some(o)) => {
                            // No critical section, relaxed ordering is fine.
                            bytes_len.fetch_sub(o.encoded_len(), Ordering::Relaxed);
                            data.push(o);
                        }
                        poped => {
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
                            match poped {
                                None => thread::park(),
                                Some(None) => break,
                                _ => unreachable!(),
                            }
                        }
                    }
                }
            }
        });

        Self {
            bytes_len,
            bytes_watermark,
            data_queue,
            exporter,
        }
    }

    pub fn flush(&self) {
        self.exporter.thread().unpark();
    }

    pub fn schedule(&self, data: Data) {
        let encoded_len = data.encoded_len();
        self.data_queue.push(Some(data));
        // No critical section, relaxed ordering is fine.
        let prev = self.bytes_len.fetch_add(encoded_len, Ordering::Relaxed);
        if prev > self.bytes_watermark {
            self.exporter.thread().unpark();
        }
    }
}

impl Drop for DataExporter {
    fn drop(&mut self) {
        // Notify the consumer that there is no more data.
        self.data_queue.push(None);
        self.exporter.thread().unpark();
    }
}

#[derive(Clone)]
pub struct Ctx {
    pub instance_id: String,
    pub exporter: Arc<DataExporter>,
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
            ctx.exporter.flush();
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
        ctx.exporter.schedule(data);

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
        ctx.exporter.schedule(data);

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
        ctx.exporter.schedule(data);

        Ok(Ok(()))
    }
}

pub fn add_to_linker<T>(
    l: &mut Linker<T>,
    f: impl (Fn(&mut T) -> &mut DataExportCtx) + Copy + Send + Sync + 'static,
) -> anyhow::Result<()> {
    Imports::add_to_linker(l, f)
}
