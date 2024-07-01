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

use crate::binary_object::{Reader, Result};
use gimli::{
    BaseAddresses, CommonInformationEntry, EhFrame, EhFrameOffset, FrameDescriptionEntry,
    UnitOffset,
};
use std::borrow::Cow;

use std::io::Write;

pub fn dump_pointer<W: Write>(w: &mut W, p: gimli::Pointer) -> Result<()> {
    match p {
        gimli::Pointer::Direct(p) => {
            write!(w, "{:#018x}", p)?;
        }
        gimli::Pointer::Indirect(p) => {
            write!(w, "({:#018x})", p)?;
        }
    }
    Ok(())
}

#[allow(clippy::unneeded_field_pattern)]
pub fn dump_cfi_instructions<R: Reader, W: Write>(
    w: &mut W,
    mut insns: gimli::CallFrameInstructionIter<R>,
    is_initial: bool,
    register_name: &dyn Fn(gimli::Register) -> Cow<'static, str>,
) -> Result<()> {
    use gimli::CallFrameInstruction::*;

    // TODO: we need to actually evaluate these instructions as we iterate them
    // so we can print the initialized state for CIEs, and each unwind row's
    // registers for FDEs.
    //
    // TODO: We should print DWARF expressions for the CFI instructions that
    // embed DWARF expressions within themselves.

    if is_initial {
        writeln!(w, "  Instructions:")?;
    }

    loop {
        match insns.next() {
            Err(e) => {
                writeln!(w, "Failed to decode CFI instruction: {}", e)?;
                return Ok(());
            }
            Ok(None) => {
                // if is_initial {
                //     writeln!(w, "  Instructions: Init State:")?;
                // }
                return Ok(());
            }
            Ok(Some(op)) => match op {
                SetLoc { address } => {
                    writeln!(w, "                DW_CFA_set_loc ({:#x})", address)?;
                }
                AdvanceLoc { delta } => {
                    writeln!(w, "                DW_CFA_advance_loc ({})", delta)?;
                }
                DefCfa { register, offset } => {
                    writeln!(
                        w,
                        "                DW_CFA_def_cfa ({}, {})",
                        register_name(register),
                        offset
                    )?;
                }
                DefCfaSf {
                    register,
                    factored_offset,
                } => {
                    writeln!(
                        w,
                        "                DW_CFA_def_cfa_sf ({}, {})",
                        register_name(register),
                        factored_offset
                    )?;
                }
                DefCfaRegister { register } => {
                    writeln!(
                        w,
                        "                DW_CFA_def_cfa_register ({})",
                        register_name(register)
                    )?;
                }
                DefCfaOffset { offset } => {
                    writeln!(w, "                DW_CFA_def_cfa_offset ({})", offset)?;
                }
                DefCfaOffsetSf { factored_offset } => {
                    writeln!(
                        w,
                        "                DW_CFA_def_cfa_offset_sf ({})",
                        factored_offset
                    )?;
                }
                DefCfaExpression { expression: _ } => {
                    writeln!(w, "                DW_CFA_def_cfa_expression (...)")?;
                }
                Undefined { register } => {
                    writeln!(
                        w,
                        "                DW_CFA_undefined ({})",
                        register_name(register)
                    )?;
                }
                SameValue { register } => {
                    writeln!(
                        w,
                        "                DW_CFA_same_value ({})",
                        register_name(register)
                    )?;
                }
                Offset {
                    register,
                    factored_offset,
                } => {
                    writeln!(
                        w,
                        "                DW_CFA_offset ({}, {})",
                        register_name(register),
                        factored_offset
                    )?;
                }
                OffsetExtendedSf {
                    register,
                    factored_offset,
                } => {
                    writeln!(
                        w,
                        "                DW_CFA_offset_extended_sf ({}, {})",
                        register_name(register),
                        factored_offset
                    )?;
                }
                ValOffset {
                    register,
                    factored_offset,
                } => {
                    writeln!(
                        w,
                        "                DW_CFA_val_offset ({}, {})",
                        register_name(register),
                        factored_offset
                    )?;
                }
                ValOffsetSf {
                    register,
                    factored_offset,
                } => {
                    writeln!(
                        w,
                        "                DW_CFA_val_offset_sf ({}, {})",
                        register_name(register),
                        factored_offset
                    )?;
                }
                Register {
                    dest_register,
                    src_register,
                } => {
                    writeln!(
                        w,
                        "                DW_CFA_register ({}, {})",
                        register_name(dest_register),
                        register_name(src_register)
                    )?;
                }
                Expression {
                    register,
                    expression: _,
                } => {
                    writeln!(
                        w,
                        "                DW_CFA_expression ({}, ...)",
                        register_name(register)
                    )?;
                }
                ValExpression {
                    register,
                    expression: _,
                } => {
                    writeln!(
                        w,
                        "                DW_CFA_val_expression ({}, ...)",
                        register_name(register)
                    )?;
                }
                Restore { register } => {
                    writeln!(
                        w,
                        "                DW_CFA_restore ({})",
                        register_name(register)
                    )?;
                }
                RememberState => {
                    writeln!(w, "                DW_CFA_remember_state")?;
                }
                RestoreState => {
                    writeln!(w, "                DW_CFA_restore_state")?;
                }
                ArgsSize { size } => {
                    writeln!(w, "                DW_CFA_GNU_args_size ({})", size)?;
                }
                NegateRaState => {
                    writeln!(w, "                DW_CFA_AARCH64_negate_ra_state")?;
                }
                Nop => {
                    writeln!(w, "                DW_CFA_nop")?;
                }
                _ => {
                    writeln!(w, "                {:?}", op)?;
                }
            },
        }
    }
}

pub fn dump_exprloc<R: Reader, W: Write>(
    w: &mut W,
    encoding: gimli::Encoding,
    data: &gimli::Expression<R>,
) -> Result<()> {
    let mut pc = data.0.clone();
    let mut space = false;
    while pc.len() != 0 {
        let pc_clone = pc.clone();
        match gimli::Operation::parse(&mut pc, encoding) {
            Ok(op) => {
                if space {
                    write!(w, " ")?;
                } else {
                    space = true;
                }
                dump_op(w, encoding, pc_clone, op)?;
            }
            Err(gimli::Error::InvalidExpression(op)) => {
                writeln!(w, "WARNING: unsupported operation 0x{:02x}", op.0)?;
                return Ok(());
            }
            Err(gimli::Error::UnsupportedRegister(register)) => {
                writeln!(w, "WARNING: unsupported register {}", register)?;
                return Ok(());
            }
            Err(gimli::Error::UnexpectedEof(_)) => {
                writeln!(w, "WARNING: truncated or malformed expression")?;
                return Ok(());
            }
            Err(e) => {
                writeln!(w, "WARNING: unexpected operation parse error: {}", e)?;
                return Ok(());
            }
        }
    }
    Ok(())
}

pub fn dump_op<R: Reader, W: Write>(
    w: &mut W,
    encoding: gimli::Encoding,
    mut pc: R,
    op: gimli::Operation<R>,
) -> Result<()> {
    let dwop = gimli::DwOp(pc.read_u8()?);
    write!(w, "{}", dwop)?;
    match op {
        gimli::Operation::Deref {
            base_type, size, ..
        } => {
            if dwop == gimli::DW_OP_deref_size || dwop == gimli::DW_OP_xderef_size {
                write!(w, " {}", size)?;
            }
            if base_type != UnitOffset(0) {
                write!(w, " type 0x{:08x}", base_type.0)?;
            }
        }
        gimli::Operation::Pick { index } => {
            if dwop == gimli::DW_OP_pick {
                write!(w, " {}", index)?;
            }
        }
        gimli::Operation::PlusConstant { value } => {
            write!(w, " {}", value as i64)?;
        }
        gimli::Operation::Bra { target } => {
            write!(w, " {}", target)?;
        }
        gimli::Operation::Skip { target } => {
            write!(w, " {}", target)?;
        }
        gimli::Operation::SignedConstant { value } => match dwop {
            gimli::DW_OP_const1s
            | gimli::DW_OP_const2s
            | gimli::DW_OP_const4s
            | gimli::DW_OP_const8s
            | gimli::DW_OP_consts => {
                write!(w, " {}", value)?;
            }
            _ => {}
        },
        gimli::Operation::UnsignedConstant { value } => match dwop {
            gimli::DW_OP_const1u
            | gimli::DW_OP_const2u
            | gimli::DW_OP_const4u
            | gimli::DW_OP_const8u
            | gimli::DW_OP_constu => {
                write!(w, " {}", value)?;
            }
            _ => {
                // These have the value encoded in the operation, eg DW_OP_lit0.
            }
        },
        gimli::Operation::Register { register } => {
            if dwop == gimli::DW_OP_regx {
                write!(w, " {}", register.0)?;
            }
        }
        gimli::Operation::RegisterOffset {
            register,
            offset,
            base_type,
        } => {
            if dwop >= gimli::DW_OP_breg0 && dwop <= gimli::DW_OP_breg31 {
                write!(w, "{:+}", offset)?;
            } else {
                write!(w, " {}", register.0)?;
                if offset != 0 {
                    write!(w, "{:+}", offset)?;
                }
                if base_type != UnitOffset(0) {
                    write!(w, " type 0x{:08x}", base_type.0)?;
                }
            }
        }
        gimli::Operation::FrameOffset { offset } => {
            write!(w, " {}", offset)?;
        }
        gimli::Operation::Call { offset } => match offset {
            gimli::DieReference::UnitRef(gimli::UnitOffset(offset)) => {
                write!(w, " 0x{:08x}", offset)?;
            }
            gimli::DieReference::DebugInfoRef(gimli::DebugInfoOffset(offset)) => {
                write!(w, " 0x{:08x}", offset)?;
            }
        },
        gimli::Operation::Piece {
            size_in_bits,
            bit_offset: None,
        } => {
            write!(w, " {}", size_in_bits / 8)?;
        }
        gimli::Operation::Piece {
            size_in_bits,
            bit_offset: Some(bit_offset),
        } => {
            write!(w, " 0x{:08x} offset 0x{:08x}", size_in_bits, bit_offset)?;
        }
        gimli::Operation::ImplicitValue { data } => {
            let data = data.to_slice()?;
            write!(w, " 0x{:08x} contents 0x", data.len())?;
            for byte in data.iter() {
                write!(w, "{:02x}", byte)?;
            }
        }
        gimli::Operation::ImplicitPointer { value, byte_offset } => {
            write!(w, " 0x{:08x} {}", value.0, byte_offset)?;
        }
        gimli::Operation::EntryValue { expression } => {
            write!(w, "(")?;
            dump_exprloc(w, encoding, &gimli::Expression(expression))?;
            write!(w, ")")?;
        }
        gimli::Operation::ParameterRef { offset } => {
            write!(w, " 0x{:08x}", offset.0)?;
        }
        gimli::Operation::Address { address } => {
            write!(w, " 0x{:08x}", address)?;
        }
        gimli::Operation::AddressIndex { index } => {
            write!(w, " 0x{:08x}", index.0)?;
        }
        gimli::Operation::ConstantIndex { index } => {
            write!(w, " 0x{:08x}", index.0)?;
        }
        gimli::Operation::TypedLiteral { base_type, value } => {
            write!(w, " type 0x{:08x} contents 0x", base_type.0)?;
            for byte in value.to_slice()?.iter() {
                write!(w, "{:02x}", byte)?;
            }
        }
        gimli::Operation::Convert { base_type } => {
            write!(w, " type 0x{:08x}", base_type.0)?;
        }
        gimli::Operation::Reinterpret { base_type } => {
            write!(w, " type 0x{:08x}", base_type.0)?;
        }
        gimli::Operation::WasmLocal { index }
        | gimli::Operation::WasmGlobal { index }
        | gimli::Operation::WasmStack { index } => {
            let wasmop = pc.read_u8()?;
            write!(w, " 0x{:x} 0x{:x}", wasmop, index)?;
        }
        gimli::Operation::Drop
        | gimli::Operation::Swap
        | gimli::Operation::Rot
        | gimli::Operation::Abs
        | gimli::Operation::And
        | gimli::Operation::Div
        | gimli::Operation::Minus
        | gimli::Operation::Mod
        | gimli::Operation::Mul
        | gimli::Operation::Neg
        | gimli::Operation::Not
        | gimli::Operation::Or
        | gimli::Operation::Plus
        | gimli::Operation::Shl
        | gimli::Operation::Shr
        | gimli::Operation::Shra
        | gimli::Operation::Xor
        | gimli::Operation::Eq
        | gimli::Operation::Ge
        | gimli::Operation::Gt
        | gimli::Operation::Le
        | gimli::Operation::Lt
        | gimli::Operation::Ne
        | gimli::Operation::Nop
        | gimli::Operation::PushObjectAddress
        | gimli::Operation::TLS
        | gimli::Operation::CallFrameCFA
        | gimli::Operation::StackValue => {}
    };
    Ok(())
}

pub fn dump_cie<R: Reader, W: Write>(
    w: &mut W,
    arch: object::Architecture,
    eh_frame: &EhFrame<R>,
    bases: &BaseAddresses,
    cie: &CommonInformationEntry<R>,
) -> Result<()> {
    fn register_name_none(_: gimli::Register) -> Option<&'static str> {
        None
    }
    let arch_register_name = match arch {
        object::Architecture::PowerPc64 => gimli::PowerPc64::register_name,
        object::Architecture::Arm | object::Architecture::Aarch64 => gimli::Arm::register_name,
        object::Architecture::I386 => gimli::X86::register_name,
        object::Architecture::X86_64 => gimli::X86_64::register_name,
        _ => register_name_none,
    };
    let register_name = &|register| match arch_register_name(register) {
        Some(name) => Cow::Borrowed(name),
        None => Cow::Owned(format!("{}", register.0)),
    };
    writeln!(w)?;
    writeln!(w, "{:#010x}: CIE", cie.offset())?;
    writeln!(w, "        length: {:#010x}", cie.entry_len())?;
    // TODO: CIE_id
    writeln!(w, "       version: {:#04x}", cie.version())?;
    // TODO: augmentation
    writeln!(w, "    code_align: {}", cie.code_alignment_factor())?;
    writeln!(w, "    data_align: {}", cie.data_alignment_factor())?;
    writeln!(
        w,
        "   ra_register: {}",
        register_name(cie.return_address_register())
    )?;
    if let Some(encoding) = cie.lsda_encoding() {
        writeln!(
            w,
            " lsda_encoding: {}/{}",
            encoding.application(),
            encoding.format()
        )?;
    }
    if let Some((encoding, personality)) = cie.personality_with_encoding() {
        write!(
            w,
            "   personality: {}/{} ",
            encoding.application(),
            encoding.format()
        )?;
        dump_pointer(w, personality)?;
        writeln!(w)?;
    }
    if let Some(encoding) = cie.fde_address_encoding() {
        writeln!(
            w,
            "  fde_encoding: {}/{}",
            encoding.application(),
            encoding.format()
        )?;
    }
    let instructions = cie.instructions(eh_frame, bases);
    dump_cfi_instructions(w, instructions, true, register_name)?;
    writeln!(w)?;
    Ok(())
}

pub fn dump_fde<R, W>(
    w: &mut W,
    arch: object::Architecture,
    eh_frame: &EhFrame<R>,
    bases: &BaseAddresses,
    cie_offset: EhFrameOffset,
    fde: &FrameDescriptionEntry<R>,
) -> Result<()>
where
    R: Reader,
    W: Write,
{
    fn register_name_none(_: gimli::Register) -> Option<&'static str> {
        None
    }
    let arch_register_name = match arch {
        object::Architecture::PowerPc64 => gimli::PowerPc64::register_name,
        object::Architecture::Arm | object::Architecture::Aarch64 => gimli::Arm::register_name,
        object::Architecture::I386 => gimli::X86::register_name,
        object::Architecture::X86_64 => gimli::X86_64::register_name,
        _ => register_name_none,
    };
    let register_name = &|register| match arch_register_name(register) {
        Some(name) => Cow::Borrowed(name),
        None => Cow::Owned(format!("{}", register.0)),
    };

    writeln!(w)?;
    writeln!(w, "{:#010x}: FDE", fde.offset())?;
    writeln!(w, "        length: {:#010x}", fde.entry_len())?;
    writeln!(w, "   CIE_pointer: {:#010x}", cie_offset.0)?;

    // TODO: symbolicate the start address like the canonical dwarfdump does.
    writeln!(w, "    start_addr: {:#018x}", fde.initial_address())?;
    writeln!(
        w,
        "    range_size: {:#018x} (end_addr = {:#018x})",
        fde.len(),
        fde.initial_address() + fde.len()
    )?;

    if let Some(lsda) = fde.lsda() {
        write!(w, "          lsda: ")?;
        dump_pointer(w, lsda)?;
        writeln!(w)?;
    }

    let instructions = fde.instructions(eh_frame, bases);
    dump_cfi_instructions(w, instructions, true, register_name)?;
    writeln!(w)?;

    Ok(())
}
