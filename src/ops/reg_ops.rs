use crate::{addressing::AddressingMode, cpu::CPU, reg::Register};

use super::op::check_flags;

pub fn tax(cpu: &mut CPU, _: AddressingMode) {
    let data = cpu.a.read();
    cpu.x.write(data);
    check_flags(cpu, data);
}

pub fn tay(cpu: &mut CPU, _: AddressingMode) {
    let data = cpu.a.read();
    cpu.y.write(data);
    check_flags(cpu, data);
}

pub fn txa(cpu: &mut CPU, _: AddressingMode) {
    let data = cpu.x.read();
    cpu.a.write(data);
    check_flags(cpu, data);
}

pub fn tya(cpu: &mut CPU, _: AddressingMode) {
    let data = cpu.y.read();
    cpu.a.write(data);
    check_flags(cpu, data);
}

pub fn tsx(cpu: &mut CPU, _: AddressingMode) {
    let data = cpu.stack.sp.read();
    cpu.x.write(data);
    check_flags(cpu, data);
}

pub fn txs(cpu: &mut CPU, _: AddressingMode) {
    let data = cpu.x.read();
    cpu.stack.sp.write(data);
    check_flags(cpu, data);
}

pub fn dex(cpu: &mut CPU, _: AddressingMode) {
    cpu.x -= 1;
    check_flags(cpu, cpu.x.read());
}

pub fn dey(cpu: &mut CPU, _: AddressingMode) {
    cpu.y -= 1;
    check_flags(cpu, cpu.y.read());
}

pub fn inx(cpu: &mut CPU, _: AddressingMode) {
    cpu.x += 1;
    check_flags(cpu, cpu.x.read());
}

pub fn iny(cpu: &mut CPU, _: AddressingMode) {
    cpu.y += 1;
    check_flags(cpu, cpu.y.read());
}
