use crate::{
    addressing::AddressingMode,
    cpu::{self, CPU},
};

fn exec_branch(cpu: &mut CPU, condition: bool) {
    let offset = cpu.read_i16();
    //info!("Branching by {} bytes", offset);
    if condition {
        if offset < 0 {
            cpu.pc.pc = cpu.pc.pc.wrapping_sub(offset.abs() as u16);
        } else {
            cpu.pc.pc = cpu.pc.pc.wrapping_add(offset as u16);
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

// SUBROUTINE OPS

pub fn jsr(cpu: &mut CPU) {
    let jump_addr = cpu.read_u16();
    let return_addr = cpu.pc.pc.wrapping_sub(1);
    cpu.stack.push_u16(return_addr);
    cpu.pc.pc = jump_addr;
}

pub fn rts(cpu: &mut CPU) {
    cpu.pc.pc = cpu.stack.pop_u16().wrapping_add(1);
}