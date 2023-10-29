use crate::{cpu::CPU, addressing::AddressingMode, reg::Register};

use super::op::check_flags;


pub fn ora(cpu: &mut CPU, mode: AddressingMode) {
    let data = cpu.read_addr(mode);
    cpu.a.write(cpu.a.read() | data);
    check_flags(cpu, data);
}

pub fn and(cpu: &mut CPU, mode: AddressingMode) {
    let data = cpu.read_addr(mode);
    cpu.a.write(cpu.a.read() & data);
    check_flags(cpu, data);
}

// L imagine using eor instead of xor
pub fn eor(cpu: &mut CPU, mode: AddressingMode) {
    let data = cpu.read_addr(mode);
    cpu.a.write(cpu.a.read() ^ data);
    check_flags(cpu, data);
}