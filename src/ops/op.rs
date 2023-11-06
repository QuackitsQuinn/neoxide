use crate::{addressing::AddressingMode, cpu::CPU, reg::Register};

use super::opcodes;

/// Delegates the execution of the next operation to the appropriate function.  
/// This function is here because a 255 line match statement is not very readable to be in cpu.rs
pub fn exec_op(cpu: &mut CPU) {
    let op = cpu.read_opbyte();
    (opcodes::OPTABLE[op as usize].op)(cpu, opcodes::OPTABLE[op as usize].mode);
}

/// No op - does nothing
pub(super) fn nop(_cpu: &mut CPU, _: AddressingMode) {}

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
pub(super) fn undoc_nop(cpu: &mut CPU, _mode: AddressingMode) {
    warn!("Undocumented opcode executed! Code: {:#X}", cpu.read(cpu.pc.read()));
}