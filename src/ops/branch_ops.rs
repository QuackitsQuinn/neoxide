use crate::{
    addressing::AddressingMode,
    cpu::{self, CPU},
};

fn exec_branch(cpu: &mut CPU, condition: bool) {
    let offset = cpu.read_u16() as i16;
    if condition {
        if offset < 0 {
            cpu.pc.pc.wrapping_sub(offset.abs() as u16);
        } else {
            cpu.pc.pc.wrapping_add(offset as u16);
        }
    }
}

pub fn jmp(cpu: &mut CPU, mode: AddressingMode) {
    let addr = cpu.get_addr(mode);
    cpu.pc.pc = addr;
}

pub fn bne(cpu: &mut CPU) {
    exec_branch(cpu, !cpu.status.is_zero());
}

pub fn beq(cpu: &mut CPU) {
    exec_branch(cpu, cpu.status.is_zero());
}

pub fn bpl(cpu: &mut CPU) {
    exec_branch(cpu, !cpu.status.is_negative());
}

pub fn bmi(cpu: &mut CPU) {
    exec_branch(cpu, cpu.status.is_negative());
}

pub fn bvc(cpu: &mut CPU) {
    exec_branch(cpu, !cpu.status.is_overflow());
}

pub fn bvs(cpu: &mut CPU) {
    exec_branch(cpu, cpu.status.is_overflow());
}

pub fn bcc(cpu: &mut CPU) {
    exec_branch(cpu, !cpu.status.is_carry());
}

pub fn bcs(cpu: &mut CPU) {
    exec_branch(cpu, cpu.status.is_carry());
}
