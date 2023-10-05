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

