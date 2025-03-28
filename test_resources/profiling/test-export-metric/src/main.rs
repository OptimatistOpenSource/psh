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

#[rustfmt::skip]
#[allow(dead_code)]
mod imports;

use imports::profiling::data_export::{
    common::FieldValue,
    metric::{export_sample, Sample},
};

fn main() {
    for i in 0..100 {
        let sample = Sample {
            name: "test-export-metric".to_string(),
            tags: vec![("tag".to_string(), "hello".to_string())],
            value: FieldValue::Int(i),
            ns_ts: None,
        };
        export_sample(&sample).expect("Failed to export sample");
    }
}
