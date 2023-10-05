use crate::cpu::CPU;



pub fn clc(cpu: &mut CPU) {
    cpu.status.set_carry(false);
}

pub fn sec(cpu: &mut CPU) {
    cpu.status.set_carry(true);
}

pub fn cli(cpu: &mut CPU) {
    cpu.status.set_interrupt(false);
}

pub fn sei(cpu: &mut CPU) {
    cpu.status.set_interrupt(true);
}

pub fn clv(cpu: &mut CPU) {
    cpu.status.set_overflow(false);
}

// no cld or sed because decimal mode is not supported on the weird dialect of 6502 used by the NES