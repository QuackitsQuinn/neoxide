use crate::{addressing::AddressingMode, cpu::CPU};

pub fn clc(cpu: &mut CPU, _: AddressingMode) {
    cpu.status.set_carry(false);
}

pub fn sec(cpu: &mut CPU, _: AddressingMode) {
    cpu.status.set_carry(true);
}

pub fn cli(cpu: &mut CPU, _: AddressingMode) {
    cpu.status.set_interrupt(false);
}

pub fn sei(cpu: &mut CPU, _: AddressingMode) {
    cpu.status.set_interrupt(true);
}

pub fn clv(cpu: &mut CPU, _: AddressingMode) {
    cpu.status.set_overflow(false);
}

// even though decimal mode is not supported, we still need to implement these because they still exist, just do nothing

pub fn cld(cpu: &mut CPU, _: AddressingMode) {
    cpu.status.set_decimal(false);
}

pub fn sed(cpu: &mut CPU, _: AddressingMode) {
    cpu.status.set_decimal(true);
}
