use crate::{cpu::CPU, reg::Register, addressing::AddressingMode};

pub fn pha(cpu: &mut CPU, _: AddressingMode) {
    let data = cpu.a.read();
    cpu.stack.push(data);
}

pub fn pla(cpu: &mut CPU, _: AddressingMode) {
    let data = cpu.stack.pop();
    cpu.a.write(data);
}

pub fn php(cpu: &mut CPU, _: AddressingMode) {
    let data = cpu.status.status;
    cpu.stack.push(data);
}

pub fn plp(cpu: &mut CPU, _: AddressingMode) {
    let data = cpu.stack.pop();
    cpu.status.status = data;
}
