use crate::{addressing::AddressingMode, cpu::CPU, reg::Register};


pub fn sta(cpu: &mut CPU, mode: AddressingMode) {
    let addr = cpu.get_addr(mode);
    cpu.write(addr, cpu.a.read());
}

pub fn stx(cpu: &mut CPU, mode: AddressingMode) {
    let addr = cpu.get_addr(mode);
    cpu.write(addr, cpu.x.read());
}

pub fn sty(cpu: &mut CPU, mode: AddressingMode) {
    let addr = cpu.get_addr(mode);
    cpu.write(addr, cpu.y.read());
}