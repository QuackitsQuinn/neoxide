use crate::{addressing::AddressingMode, cpu::CPU, reg::Register};

use super::{opcode::Operation, opcodes};

/// Delegates the execution of the next operation to the appropriate function.  
/// This function is here because a 255 line match statement is not very readable to be in cpu.rs
pub fn exec_op(cpu: &mut CPU) {
    let op = cpu.read_opbyte();
    let init_pc = cpu.pc.read() - 1;
    let ex_op = opcodes::OPTABLE[op as usize];
    #[cfg(debug_assertions)] // only log if we are in debug mode
    log_opinfo(cpu, ex_op, init_pc);
}

fn log_opinfo(cpu: &mut CPU, ex_op: Operation, init_pc: u16) {
    info!(
        "Executing opcode: {:#X} ({}:{}) at address {:#4X}",
        ex_op.code,
        ex_op.name,
        ex_op.mode,
        cpu.pc.read() - 1
    );
    (ex_op.op)(cpu, ex_op.mode);
    if ex_op.optype != "branch" {
        let pcsub = cpu.pc.read().wrapping_sub(init_pc);
        info!("Consumed {:#X} bytes", pcsub);
        if pcsub != ex_op.length as u16 {
            warn!("Consumed {} bytes, expected {}", pcsub, ex_op.length);
        }
    } else {
        info!("Jumped to {:#X}", cpu.pc.read());
    }
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
    warn!(
        "Undocumented opcode executed! Code: {:#X}",
        cpu.read(cpu.pc.read())
    );
}
