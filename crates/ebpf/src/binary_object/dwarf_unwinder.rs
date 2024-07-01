/// This file is inspired by the paper of Reliable and Fast DWARF-Based Stack Unwinnding.
/// 
/// Informal semantics of DWARF instructions. Below we report the DWARF instructins used for
/// CFI description, descriptions have been reworded for brevity and clarity. Since we abstract from
/// the underlying file format here, we omit variantions differing only on the number of bytes of their
/// operand, e.g. advance_loc1 VS. advance_loc2.
/// 
/// - set_loc(loc): start a new table row from address `loc`;
/// - advance_loc(delta): start a new table row at address `prev_loc` + `delta`;
/// - def_cfa(reg, offset): sets this row's CFA at `(%reg + offset)`;
/// - def_cfa_register(reg): sets CFA at `(%reg + prev_offset)`;
/// - def_cfa_offset(offset): sets CFA at `(%prev_reg + offset)`;
/// - def_cfa_expression(expr): sets CFA as the result of `expr`;
/// - undefined(reg): sets the register `%reg` as undefined in this row;
/// - same_value(reg): declares that the register `%reg` hasn't been touched, or was restored to its
///                    previous value, in this row. An unwinding procedure can leave it as-is;
/// - offset(reg, offset): the value of the register `%reg` is stored in memory at the address `CFA + offset`;
/// - val_offset(reg, offset): the value of the register `%reg` is the value `CFA + offset`;
/// - register(reg, model): the register `%reg` has, in this row, the value of `%model`;
/// - expression(reg, expr): the value of `%reg` is stored in memory at the address defined by `expr`;
/// - val_expression(reg, expr): `%reg` has the value of `expr`;
/// - restore(reg): `%reg` has the same value as in this FDE's preamble (CIE) in this row. We do not
///                 support this instruction, as it requires considerable boilerplate to differentiate CIE
///                 (preamble) and FDE (body) instructions, and is not used in practice (see Section 3.2);
/// - remeber_state(): push the state of all the registers of this row on a state-saving stack;
/// - restore_state(): pop an entry of the state-saving stack, and restore all registers in this row to
///                    the value held in the stack record;
/// - nop(): do nothing (padding).