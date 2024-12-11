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

use host_op_perf::PerfCtx;
use host_op_system::SysCtx;
use wasmtime::component::ResourceTable;
use wasmtime_wasi::{WasiCtx, WasiView};

use super::DataExportCtx;

pub struct PshState {
    #[allow(dead_code)]
    pub name: String,
    pub table: ResourceTable,
    pub wasi_ctx: WasiCtx,
    pub perf_ctx: PerfCtx,
    pub sys_ctx: SysCtx,
    pub data_export_ctx: DataExportCtx,
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
