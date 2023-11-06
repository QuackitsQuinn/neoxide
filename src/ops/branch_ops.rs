use crate::{addressing::AddressingMode, cpu::CPU};

fn exec_branch(cpu: &mut CPU, condition: bool) {
    let offset = cpu.read_i8();
    info!("Branching by {} bytes", offset);
    if condition {
        if offset < 0 {
            cpu.pc.pc = cpu.pc.pc.wrapping_sub(offset.abs() as u16);
        } else {
            cpu.pc.pc = cpu.pc.pc.wrapping_add(offset as u16);
        }
    }
}

pub fn jmp(cpu: &mut CPU, mode: AddressingMode) {
    let addr = if AddressingMode::Absolute == mode {
        cpu.read_u16()
    } else {
        cpu.get_addr(mode)
    };
    println!("JMP to {:X}", addr);
    cpu.pc.pc = addr;
}

pub fn bne(cpu: &mut CPU, _: AddressingMode) {
    exec_branch(cpu, !cpu.status.is_zero());
}

pub fn beq(cpu: &mut CPU, _: AddressingMode) {
    exec_branch(cpu, cpu.status.is_zero());
}

pub fn bpl(cpu: &mut CPU, _: AddressingMode) {
    exec_branch(cpu, !cpu.status.is_negative());
}

pub fn bmi(cpu: &mut CPU, _: AddressingMode) {
    exec_branch(cpu, cpu.status.is_negative());
}

pub fn bvc(cpu: &mut CPU, _: AddressingMode) {
    exec_branch(cpu, !cpu.status.is_overflow());
}

pub fn bvs(cpu: &mut CPU, _: AddressingMode) {
    exec_branch(cpu, cpu.status.is_overflow());
}

pub fn bcc(cpu: &mut CPU, _: AddressingMode) {
    exec_branch(cpu, !cpu.status.is_carry());
}

pub fn bcs(cpu: &mut CPU, _: AddressingMode) {
    exec_branch(cpu, cpu.status.is_carry());
}

// SUBROUTINE OPS

pub fn jsr(cpu: &mut CPU, _: AddressingMode) {
    let jump_addr = cpu.read_u16();
    let return_addr = cpu.pc.pc.wrapping_sub(1);
    cpu.stack.push_u16(return_addr);
    cpu.pc.pc = jump_addr - 1;
}

pub fn rts(cpu: &mut CPU, _: AddressingMode) {
    cpu.pc.pc = cpu.stack.pop_u16().wrapping_add(1);
}

// break

pub fn brk(cpu: &mut CPU, _: AddressingMode) {
    cpu.pc.pc += 1;
    cpu.stack.push_u16(cpu.pc.pc);
    cpu.stack.push(cpu.status.status | 0b0011_0000);
    cpu.status.set_interrupt(true);
    cpu.pc.pc = cpu.read_u16();
}

pub fn rti(cpu: &mut CPU, _: AddressingMode) {
    cpu.status.status = cpu.stack.pop() & 0b1101_1111;
    cpu.pc.pc = cpu.stack.pop_u16();
}
