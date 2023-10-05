use crate::{addressing::AddressingMode, cpu::CPU, reg::Register};

use super::op::check_flags;

/// Executes the LDA instruction with the given addressing mode and CPU
pub fn lda(cpu: &mut CPU, mode: AddressingMode) {
    let data = cpu.read_addr(mode);
    cpu.a.write(data);
    check_flags(cpu, data);
}

pub fn ldx(cpu: &mut CPU, mode: AddressingMode) {
    let data = cpu.read_addr(mode);
    cpu.x.write(data);
    check_flags(cpu, data);
}

pub fn ldy(cpu: &mut CPU, mode: AddressingMode) {
    let data = cpu.read_addr(mode);
    cpu.y.write(data);
    check_flags(cpu, data);
}
