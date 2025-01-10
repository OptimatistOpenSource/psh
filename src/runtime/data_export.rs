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

use std::{collections::VecDeque, sync::Arc};

use chrono::{TimeZone, Utc};
use profiling::data_export::measurement::Point;
use profiling::data_export::metric::Sample;
use profiling::data_export::types::FieldValue as WitFieldValue;
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

#[derive(Clone)]
pub struct DataExportBuf {
    watermark: usize,
    len: usize,
    encoded_reqs: VecDeque<Vec<u8>>,
}

impl DataExportBuf {
    pub fn new(len: usize) -> Self {
        Self {
            watermark: len,
            len: 0,
            encoded_reqs: VecDeque::with_capacity(len),
        }
    }

    pub fn push_back(&mut self, req: &ExportDataReq) -> bool {
        let enc_len = req.encoded_len();

        if self.len + enc_len > self.watermark {
            return false;
        }

        self.encoded_reqs.push_back(req.encode_to_vec());
        self.len += enc_len;
        true
    }

    pub fn pop_front(&mut self) -> Option<ExportDataReq> {
        let encoded_req = self.encoded_reqs.pop_front()?;
        self.len -= encoded_req.len();
        let k = ExportDataReq::decode(encoded_req.as_slice()).unwrap();
        Some(k)
    }
}

fn schedule_message(ctx: &mut Ctx, message: ExportDataReq) {
    if ctx.buf.push_back(&message) {
        return;
    }

    let mut task_id = None;
    let mut data = Vec::with_capacity(ctx.buf.len);

    while let Some(mut req) = ctx.buf.pop_front() {
        task_id.get_or_insert(req.task_id);
        data.append(&mut req.data);
    }

    task_id.map(|task_id| {
        let merged = ExportDataReq { task_id, data };
        let mut rpc_client = ctx.rpc_client.clone();
        ctx.exporter_rt
            .spawn(async move { rpc_client.export_data(merged).await })
    });

    schedule_message(ctx, message)
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
        while let Some(req) = self.buf.pop_front() {
            self.exporter_rt.spawn({
                let mut rpc_client = self.rpc_client.clone();
                async move { rpc_client.export_data(req).await }
            });
        }
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

impl profiling::data_export::types::Host for DataExportCtx {}

impl profiling::data_export::file::Host for DataExportCtx {
    fn export_bytes(&mut self, bytes: Vec<u8>) -> wasmtime::Result<Result<(), String>> {
        let Some(ctx) = &mut self.ctx else {
            return Ok(Ok(()));
        };

        let message = ExportDataReq {
            task_id: ctx.task_id.clone(),
            data: vec![Data {
                ty: DataType::File as _,
                bytes,
            }],
        };
        schedule_message(ctx, message);

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
        let message = ExportDataReq {
            task_id: ctx.task_id.clone(),
            data: vec![Data {
                ty: DataType::LineProtocol as _,
                bytes,
            }],
        };
        schedule_message(ctx, message);

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
        let message = ExportDataReq {
            task_id: ctx.task_id.clone(),
            data: vec![Data {
                ty: DataType::LineProtocol as _,
                bytes,
            }],
        };
        schedule_message(ctx, message);

        Ok(Ok(()))
    }
}

pub fn add_to_linker<T>(
    l: &mut Linker<T>,
    f: impl (Fn(&mut T) -> &mut DataExportCtx) + Copy + Send + Sync + 'static,
) -> anyhow::Result<()> {
    Imports::add_to_linker(l, f)
}
