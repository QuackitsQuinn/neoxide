use crate::{addressing::AddressingMode, cpu::CPU};

use super::{
    bit_ops::{and, eor, ora},
    branch_ops::{bcc, bcs, beq, bmi, bne, bpl, bvc, bvs, jmp, jsr, rts},
    load_ops::{lda, ldx, ldy},
    reg_ops::{dex, dey, inx, iny, tax, tay, tsx, txa, txs, tya},
    stack_ops::{pha, php, pla, plp},
    status_ops::{clc, cli, clv, sec, sei},
    store_ops::{sta, stx, sty},
};
/// Delegates the execution of the next operation to the appropriate function.  
/// This function is here because a 255 line match statement is not very readable to be in cpu.rs
pub fn exec_op(cpu: &mut CPU) {
    let op = cpu.read_opbyte();
    //info!("Executing opcode: {:#X}", op);
}

/// No op - does nothing
fn nop(_cpu: &mut CPU, _: AddressingMode) {}

/// Checks the data for zero and negative flags and sets them accordingly
pub(super) fn check_flags(cpu: &mut CPU, data: u8) {
    if data == 0 {
        cpu.status.set_zero(true);
    } else {
        cpu.status.set_zero(false);
    }

    if data & 0x80 == 0x80 {
        cpu.status.set_negative(true);
    } else {
        cpu.status.set_negative(false);
    }
}
