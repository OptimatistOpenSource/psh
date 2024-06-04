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
// You should have received a copy of the GNU Lesser General Public License along with Perf-event-rs. If not,
// see <https://www.gnu.org/licenses/>.

use object::ObjectSection;
use std::fmt::{self, Debug, Display};
use std::io;
use std::path::Path;
use std::result;

pub mod dwarf_cie_fde;
pub mod dwarf_dump;
pub mod elf_symbol;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    GimliError(gimli::Error),
    ObjectError(object::read::Error),
    IoError,
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> ::std::result::Result<(), fmt::Error> {
        Debug::fmt(self, f)
    }
}

impl From<gimli::Error> for Error {
    fn from(err: gimli::Error) -> Self {
        Error::GimliError(err)
    }
}

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Self {
        Error::IoError
    }
}

impl From<object::read::Error> for Error {
    fn from(err: object::read::Error) -> Self {
        Error::ObjectError(err)
    }
}

pub type Result<T> = result::Result<T, Error>;

pub trait Reader: gimli::Reader<Offset = usize> + Send + Sync {}

impl<'input, Endian> Reader for gimli::EndianSlice<'input, Endian> where
    Endian: gimli::Endianity + Send + Sync
{
}

#[derive(Debug, Default)]
struct RelocationMap(object::read::RelocationMap);

impl RelocationMap {
    fn add(&mut self, file: &object::File, section: &object::Section) {
        for (offset, relocation) in section.relocations() {
            if let Err(e) = self.0.add(file, offset, relocation) {
                eprintln!(
                    "Relocation error for section {} at offset 0x{:08x}: {}",
                    section.name().unwrap(),
                    offset,
                    e
                );
            }
        }
    }
}

impl<'a> gimli::read::Relocate for &'a RelocationMap {
    fn relocate_address(&self, offset: usize, value: u64) -> gimli::Result<u64> {
        Ok(self.0.relocate(offset as u64, value))
    }

    fn relocate_offset(&self, offset: usize, value: usize) -> gimli::Result<usize> {
        <usize as gimli::ReaderOffset>::from_u64(self.0.relocate(offset as u64, value as u64))
    }
}

type Relocate<'a, R> = gimli::RelocateReader<R, &'a RelocationMap>;

impl<'a, R: Reader> Reader for Relocate<'a, R> {}

// dwarf
pub struct ElfFile<'a> {
    pub path: &'a Path,
    pub file: object::File<'a>,
    pub endian: gimli::RunTimeEndian,
}

#[derive(Debug)]
pub enum BpfCfaType {
    CfaTypeUndefined = 0,
    CfaTypeRbp = 1,
    CfaTypeRsp = 2,
    CfaTypeExpression = 3,
    CfaTypeEndFdeMarker = 4,
    // FIXME (Chengdong Li): reorder the types
    CfaTypeExprRbpDeref = 5,
    CfaTypeExprRspDeref = 6,
}

#[derive(Debug)]
pub enum BpfRbpType {
    RbpRuleOffsetSetUnchanged = 0,
    RbpRuleOffset = 1,
    RbpRuleRegister = 2,
    RbpTypeExpression = 3,
    RbpTypeUndefinedReturnAddress = 4,
    // FIXME (Chengdong Li): reorder the types
    RbpTypeExprRbp = 5,
    RbpTypeExprRsp = 6,
    RbpTypeExprUnknown = 7,
}

#[derive(Debug)]
pub struct CompactUnwindTableRow {
    pub pc: u64,
    #[cfg(target_arch = "aarch64")]
    pub lr_offset: i16,
    pub cfa_type: BpfCfaType,
    pub rbp_type: BpfRbpType,
    pub cfa_offset: i16,
    pub rbp_offset: i16,
}

impl Display for CompactUnwindTableRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = write!(
            f,
            "pc: {:#x} cfa_type: {:?} rbp_type: {:?} cfa_offset: {} rbp_offset: {}",
            self.pc, self.cfa_type, self.rbp_type, self.cfa_offset, self.rbp_offset
        );

        #[cfg(target_arch = "aarch64")]
        let r = write!(f, " lr_offset: {}", self.lr_offset);

        r
    }
}

#[derive(Debug)]
pub struct Symbol {
    start: u64,
    end: u64,
    //r#type: SymbolType,
    symbol: String,
    // attributes: String,
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#x} {:#x} {}", self.start, self.end, self.symbol)
    }
}
