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

use gimli::{DwOp, Register, Section, UnitOffset, UnwindContext, UnwindSection};
use object::{Object, ObjectSection};
use std::borrow::Cow;
use std::collections::HashMap;
use std::io::Stdout;
use std::io::Write;
use std::mem;
use std::ops::Add;

use typed_arena::Arena;

use crate::binary_object::dwarf_dump::dump_exprloc;
use crate::binary_object::{
    BpfCfaType, BpfRbpType, CompactUnwindTableRow, ElfFile, Reader, Relocate, RelocationMap, Result,
};

use super::dwarf_dump::dump_cie;
use super::dwarf_dump::dump_fde;

fn load_file_section<'input, 'arena, Endian: gimli::Endianity>(
    id: gimli::SectionId,
    file: &object::File<'input>,
    endian: Endian,
    is_dwo: bool,
    arena_data: &'arena Arena<Cow<'input, [u8]>>,
    arena_relocations: &'arena Arena<RelocationMap>,
) -> Result<Relocate<'arena, gimli::EndianSlice<'arena, Endian>>> {
    let mut relocations = RelocationMap::default();
    let name = if is_dwo {
        id.dwo_name()
    } else if file.format() == object::BinaryFormat::Xcoff {
        id.xcoff_name()
    } else {
        Some(id.name())
    };

    let data = match name.and_then(|name| file.section_by_name(name)) {
        Some(ref section) => {
            // DWO sections never have relocations, so don't bother.
            if !is_dwo {
                relocations.add(file, section);
            }
            section.uncompressed_data()?
        }
        // Use a non-zero capacity so that `ReaderOffsetId`s are unique.
        None => Cow::Owned(Vec::with_capacity(1)),
    };
    let data_ref = arena_data.alloc(data);
    let section = gimli::EndianSlice::new(data_ref, endian);
    let relocations = arena_relocations.alloc(relocations);
    Ok(Relocate::new(section, relocations))
}

// DWARF CFA expressions that we recognize.

// PLT_TYPE1 is equivalent to: sp + 8 + ((((ip & 15) >= 11)) << 3.
static PTL_TYPE1: [DwOp; 10] = [
    gimli::DW_OP_breg7,
    gimli::DW_OP_const1u,
    gimli::DW_OP_breg16,
    gimli::DW_OP_lit15,
    gimli::DW_OP_and,
    gimli::DW_OP_lit11,
    gimli::DW_OP_ge,
    gimli::DW_OP_lit3,
    gimli::DW_OP_shl,
    gimli::DW_OP_plus,
];

// PTL_TYPE2 is quivalent to: sp + 8 + ((((ip & 15) >= 10)) << 3.
static PTL_TYPE2: [DwOp; 10] = [
    gimli::DW_OP_breg7,
    gimli::DW_OP_const1u,
    gimli::DW_OP_breg16,
    gimli::DW_OP_lit15,
    gimli::DW_OP_and,
    gimli::DW_OP_lit10,
    gimli::DW_OP_ge,
    gimli::DW_OP_lit3,
    gimli::DW_OP_shl,
    gimli::DW_OP_plus,
];

pub enum DwarfCfaExpressionPltType {
    Unknown = 0,
    Type1 = 1,
    Type2 = 2,
}

pub enum PltThreshold {
    Unknown = 0,
    Threshold1 = 11,
    Threshold2 = 10,
}

fn dwarf_cfa_expr_plt_type<R: Reader>(
    expr: &gimli::Expression<R>,
) -> Result<DwarfCfaExpressionPltType> {
    #[cfg(target_arch = "x86_64")]
    {
        let mut bytes = expr.0.clone();
        let max_size = PTL_TYPE1.len().max(PTL_TYPE2.len());
        let mut bytecode = vec![0u8; max_size];

        if max_size <= bytes.len() {
            let mut i: usize = 0;
            loop {
                let b = bytes.read_u8().unwrap();
                if b == 0x0 {
                    continue;
                }
                bytecode[i] = b;
                i = i.add(1);
                if i >= max_size {
                    break;
                }
            }
            fn plt_type_equal(bytecode: &[u8], plt_type: &[DwOp; 10]) -> bool {
                let mut i = 0;
                for b in plt_type {
                    if DwOp(bytecode[i]) != *b {
                        break;
                    }
                    i = i.add(1);
                }
                i == PTL_TYPE1.len()
            }

            if plt_type_equal(&bytecode, &PTL_TYPE1) {
                return Ok(DwarfCfaExpressionPltType::Type1);
            }

            if plt_type_equal(&bytecode, &PTL_TYPE2) {
                return Ok(DwarfCfaExpressionPltType::Type2);
            }
        }
    }

    // TODO (Chengdong Li) Support aarch64.

    Ok(DwarfCfaExpressionPltType::Unknown)
}

fn x86_64_breg_offset_deref<R: Reader>(
    expr: &gimli::Expression<R>,
    encoding: gimli::Encoding,
) -> Result<(BpfCfaType, i16)> {
    let mut bytes = expr.0.clone();

    // for x86_64, also support rbp/rsp+offset deref expression
    // From our observation, such expression is consistent of two operations.
    let (cfa_type, offset) = match gimli::Operation::parse(&mut bytes, encoding)? {
        gimli::Operation::RegisterOffset {
            register,
            offset,
            base_type,
        } => {
            let cfa_type = if register == Register(6) {
                // rbp + offset case
                BpfCfaType::CfaTypeExprRbpDeref
            } else if register == Register(7) {
                // rsp + offset case
                BpfCfaType::CfaTypeExprRspDeref
            } else {
                // unsupported register.
                return Ok((BpfCfaType::CfaTypeUndefined, -1i16));
            };
            if base_type != UnitOffset(0) {
                return Ok((BpfCfaType::CfaTypeUndefined, -1i16));
            }
            (cfa_type, offset)
        }
        _ => {
            return Ok((BpfCfaType::CfaTypeUndefined, -1i16));
        }
    };

    // parse next operation to see if it is Deref operation.
    match gimli::Operation::parse(&mut bytes, encoding)? {
        gimli::Operation::Deref {
            base_type: _,
            size: _,
            space: _,
        } => {
            if bytes.len() > 0 {
                // FIXME (Chengdong Li) we don't support it yet if there is remain bytes.
                Ok((BpfCfaType::CfaTypeUndefined, -1i16))
            } else {
                Ok((cfa_type, offset as i16))
            }
        }
        _ => {
            // unsuppored operation
            Ok((BpfCfaType::CfaTypeUndefined, -1i16))
        }
    }
}

fn x86_rbp_expr_parse<R: Reader>(
    expr: &gimli::Expression<R>,
    encoding: gimli::Encoding,
) -> Result<(BpfRbpType, i16)> {
    let mut bytes = expr.0.clone();
    // for x86_64, also support rbp/rsp+offset expression
    let (rbp_type, offset) = match gimli::Operation::parse(&mut bytes, encoding)? {
        gimli::Operation::RegisterOffset {
            register,
            offset,
            base_type,
        } => {
            let rbp_type = if register == Register(6) {
                // rbp + offset case
                BpfRbpType::RbpTypeExprRbp
            } else if register == Register(7) {
                // rsp + offset case
                BpfRbpType::RbpTypeExprRsp
            } else {
                // unsupported register.
                return Ok((BpfRbpType::RbpTypeExprUnknown, -1i16));
            };
            if base_type != UnitOffset(0) || bytes.len() > 0 {
                return Ok((BpfRbpType::RbpTypeExprUnknown, -1i16));
            }
            (rbp_type, offset)
        }
        _ => {
            return Ok((BpfRbpType::RbpTypeExprUnknown, -1i16));
        }
    };

    Ok((rbp_type, offset as i16))
}

pub fn parse_elf_eh_frame(
    elf: &ElfFile,
    w: &mut Option<Stdout>,
) -> Result<Vec<CompactUnwindTableRow>> {
    let arena_data = Arena::new();
    let arena_relocations = Arena::new();

    let load_section = |id: gimli::SectionId| -> Result<_> {
        load_file_section(
            id,
            &elf.file,
            elf.endian,
            false,
            &arena_data,
            &arena_relocations,
        )
    };

    let mut eh_frame = gimli::EhFrame::load(load_section)?;

    let address_size = elf
        .file
        .architecture()
        .address_size()
        .map(|adrs| adrs.bytes())
        .unwrap_or(mem::size_of::<usize>() as u8);
    eh_frame.set_address_size(address_size);

    // There are other things we could match but currently don't
    #[allow(clippy::single_match)]
    match elf.file.architecture() {
        object::Architecture::Aarch64 => eh_frame.set_vendor(gimli::Vendor::AArch64),
        _ => {}
    }

    let mut bases = gimli::BaseAddresses::default();
    if let Some(section) = elf.file.section_by_name(".eh_frame_hdr") {
        bases = bases.set_eh_frame_hdr(section.address());
    }
    if let Some(section) = elf.file.section_by_name(".eh_frame") {
        bases = bases.set_eh_frame(section.address());
    }
    if let Some(section) = elf.file.section_by_name(".text") {
        bases = bases.set_text(section.address());
    }
    if let Some(section) = elf.file.section_by_name(".got") {
        bases = bases.set_got(section.address());
    }

    let mut ctx = UnwindContext::new();
    let mut cies = HashMap::new();
    let mut entries = eh_frame.entries(&bases);

    let mut compact_unwind_table_rows = Vec::<CompactUnwindTableRow>::new();

    loop {
        match entries.next()? {
            None => break,
            Some(gimli::CieOrFde::Cie(cie)) => {
                if let Some(w) = w {
                    dump_cie(w, elf.file.architecture(), &eh_frame, &bases, &cie)?
                }
            }
            Some(gimli::CieOrFde::Fde(partial)) => {
                let cie_offset = partial.cie_offset();
                let fde = match partial.parse(|_, bases, o| {
                    cies.entry(o)
                        .or_insert_with(|| eh_frame.cie_from_offset(bases, o))
                        .clone()
                }) {
                    Ok(fde) => fde,
                    Err(e) => {
                        if let Some(w) = w {
                            writeln!(w, "Failed to parse FDE: {}", e)?;
                        }
                        continue;
                    }
                };
                if let Some(w) = w {
                    dump_fde(
                        w,
                        elf.file.architecture(),
                        &eh_frame,
                        &bases,
                        cie_offset,
                        &fde,
                    )?;
                }

                // Interpret all DWARF instructions of this Frame Description Entry (FDE). This gives us an unwind
                // table that contains rules for retrieving registers at every instruction address.
                let mut table = fde.rows(&eh_frame, &bases, &mut ctx)?;

                loop {
                    match table.next_row() {
                        Ok(None) => break,
                        Ok(Some(row)) => {
                            let (cfa_type, cfa_offset) = match row.cfa() {
                                gimli::CfaRule::RegisterAndOffset { register, offset } => {
                                    match register {
                                        #[cfg(target_arch = "x86_64")]
                                        Register(7) => (BpfCfaType::CfaTypeRsp, *offset as i16),
                                        #[cfg(target_arch = "x86_64")]
                                        Register(6) => (BpfCfaType::CfaTypeRbp, *offset as i16),
                                        // TODO (Chengdong Li) observed using r10 as cfa register in x86_64
                                        // DW_CFA_def_cfa (r10, 0)

                                        // TODO (Chengdong Li) support aarch64 platform.
                                        _ => (BpfCfaType::CfaTypeUndefined, *offset as i16),
                                    }
                                }
                                gimli::CfaRule::Expression(unwind_expression) => {
                                    let expression = unwind_expression.get(&eh_frame)?;
                                    if let Some(w) = w {
                                        writeln!(w)?;
                                        writeln!(w, "{:?}", row)?;
                                        dump_exprloc(w, fde.cie().encoding(), &expression)?;
                                        writeln!(w, "\n")?;
                                    }
                                    match dwarf_cfa_expr_plt_type(&expression)? {
                                        DwarfCfaExpressionPltType::Unknown => {
                                            if cfg!(target_arch = "x86_64") {
                                                x86_64_breg_offset_deref(
                                                    &expression,
                                                    fde.cie().encoding(),
                                                )?
                                            } else {
                                                (BpfCfaType::CfaTypeUndefined, -1_i16)
                                            }
                                        }
                                        DwarfCfaExpressionPltType::Type1 => (
                                            BpfCfaType::CfaTypeExpression,
                                            PltThreshold::Threshold1 as i16,
                                        ),
                                        DwarfCfaExpressionPltType::Type2 => (
                                            BpfCfaType::CfaTypeExpression,
                                            PltThreshold::Threshold2 as i16,
                                        ),
                                    }
                                }
                            };
                            let (rbp_type, rbp_offset) = if cfg!(target_arch = "x86_64") {
                                // on x86_64, r6 is the rbp register
                                let rbp = Register(6);
                                match row.register(rbp) {
                                    gimli::RegisterRule::Offset(offset) => {
                                        (BpfRbpType::RbpRuleOffset, offset as i16)
                                    },
                                    gimli::RegisterRule::Register(r) => {
                                        (BpfRbpType::RbpRuleRegister, r.0 as i16)
                                    },
                                    gimli::RegisterRule::Expression(expr) => {
                                        let expression = expr.get(&eh_frame)?;
                                        if let Some(w) = w {
                                            write!(w, "Register(rbp) expression: ")?;
                                            dump_exprloc(w, fde.cie().encoding(), &expression)?;
                                            writeln!(w)?;
                                        }
                                        x86_rbp_expr_parse(&expression, fde.cie().encoding())?
                                    },
                                    gimli::RegisterRule::ValExpression(_) |
                                    gimli::RegisterRule::Architectural |
                                    gimli::RegisterRule::Constant(_) |
                                    gimli::RegisterRule::Undefined |
                                    gimli::RegisterRule::SameValue |
                                    // FIXME (Chengdong Li): it can be supported in appropriately.
                                    gimli::RegisterRule::ValOffset(_) => {
                                        (BpfRbpType::RbpRuleOffsetSetUnchanged, -1_i16)
                                    }
                                    _ => (BpfRbpType::RbpRuleOffsetSetUnchanged, -1_i16),
                                }
                            } else {
                                (BpfRbpType::RbpRuleOffsetSetUnchanged, -1_i16)
                            };

                            let compact_unwind_table_row = CompactUnwindTableRow {
                                pc: row.start_address(),
                                cfa_type,
                                cfa_offset,
                                rbp_type,
                                rbp_offset,
                            };
                            if let Some(w) = w {
                                writeln!(w, "{}", compact_unwind_table_row)?;
                            }
                            compact_unwind_table_rows.push(compact_unwind_table_row);
                        }
                        Err(gimli::Error::UnknownCallFrameInstruction(_)) => continue,
                        // NOTE: Temporary workaround for https://github.com/gimli-rs/gimli/pull/487
                        Err(gimli::Error::TooManyRegisterRules) => continue,
                        Err(e) => return Err(e.into()),
                    }
                }
            }
        }
    }
    Ok(compact_unwind_table_rows)
}

#[test]
fn test_parse_elf_eh_frame() {
    #[allow(unused_imports)]
    use std::io::stdout;
    use std::mem::size_of;

    // comp_unwind_table len: 287504, total_size: 4600064
    // let target = std::path::Path::new("/usr/sbin/mysqld");
    // comp_unwind_table len: 185131, total_size: 2962096
    // let target = std::path::Path::new("/usr/lib/jvm/java-11-openjdk-amd64/lib/server/libjvm.so");
    // comp_unwind_table len: 1501, total_size: 24016
    // let target = std::path::Path::new("/usr/bin/ls");
    // comp_unwind_table len: 25444, total_size: 407104
    let target = std::path::Path::new("/usr/lib/x86_64-linux-gnu/libc.so.6");
    // comp_unwind_table len: 80905, total_size: 1294480
    // let target = std::path::Path::new("/usr/lib/postgresql/14/bin/postgres");
    // comp_unwind_table len: 107294, total_size: 1716704
    // let target = std::path::Path::new("/usr/lib/virtualbox/UICommon.so");
    let file = std::fs::File::open(target).unwrap();
    let file = unsafe { memmap2::Mmap::map(&file).unwrap() };
    let file = object::File::parse(&*file).unwrap();
    let endian = if file.is_little_endian() {
        gimli::RunTimeEndian::Little
    } else {
        gimli::RunTimeEndian::Big
    };
    let elf_file = ElfFile {
        path: &target,
        file,
        endian,
    };

    //let mut w = Some(stdout());
    let mut w = None;

    match parse_elf_eh_frame(&elf_file, &mut w) {
        Ok(comp_unwind_table) => {
            println!(
                "comp_unwind_table len: {}, total_size: {}",
                comp_unwind_table.len(),
                comp_unwind_table.len() * size_of::<CompactUnwindTableRow>()
            );
            assert!(comp_unwind_table.len() > 20000)
        }
        Err(e) => assert!(false, "{}", e.to_string()),
    }
}
