use crate::{cpu::CPU, reg::Register};

use super::op::check_flags;

pub fn tax(cpu: &mut CPU) {
    let data = cpu.a.read();
    cpu.x.write(data);
    check_flags(cpu, data);
}

pub fn tay(cpu: &mut CPU) {
    let data = cpu.a.read();
    cpu.y.write(data);
    check_flags(cpu, data);
}

pub fn txa(cpu: &mut CPU) {
    let data = cpu.x.read();
    cpu.a.write(data);
    check_flags(cpu, data);
}

pub fn tya(cpu: &mut CPU) {
    let data = cpu.y.read();
    cpu.a.write(data);
    check_flags(cpu, data);
}

pub fn tsx(cpu: &mut CPU) {
    let data = cpu.stack.sp.read();
    cpu.x.write(data);
    check_flags(cpu, data);
}

pub fn txs(cpu: &mut CPU) {
    let data = cpu.x.read();
    cpu.stack.sp.write(data);
    check_flags(cpu, data);
}

pub fn dex(cpu: &mut CPU) {
    cpu.x-=1;
    check_flags(cpu, cpu.x.read());
}

pub fn dey(cpu: &mut CPU) {
    cpu.y-=1;
    check_flags(cpu, cpu.y.read());
}

pub fn inx(cpu: &mut CPU) {
    cpu.x+=1;
    check_flags(cpu, cpu.x.read());
}

pub fn iny(cpu: &mut CPU) {
    cpu.y+=1;
    check_flags(cpu, cpu.y.read());
}
