use crate::{addressing::AddressingMode, constant::PGRM_LOAD_OFFSET, cpu::CPU};

fn exec_branch(cpu: &mut CPU, condition: bool) {
    let offset = cpu.read_i8();
    let pc = cpu.pc.pc;
    //info!("Branching by {} bytes", offset);
    if condition {
        if offset < 0 {
            cpu.pc.pc = cpu.pc.pc.wrapping_sub(offset.unsigned_abs() as u16);
        } else {
            cpu.pc.pc = cpu.pc.pc.wrapping_add(offset as u16);
        }
    }
    if cpu.pc.pc < PGRM_LOAD_OFFSET {
        warn!(
            "Jumped to address {:#X} which is before the program load offset",
            cpu.pc.pc
        );
        warn!("Instruction addr: {:#X}", pc - 1);
    }
}

pub fn jmp(cpu: &mut CPU, mode: AddressingMode) {
    let addr = if AddressingMode::Absolute == mode {
        cpu.read_u16()
    } else {
        cpu.get_addr(mode)
    };
    if addr < PGRM_LOAD_OFFSET {
        warn!(
            "Jumped to address {:#X} which is before the program load offset",
            addr
        );
        warn!("Instruction addr: {:#X}", cpu.pc.pc - 1);
    }
    cpu.pc.pc = addr - 1;
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
    let return_addr = cpu.stack.pop_u16();
    if return_addr == 0x0 {
        warn!("Stack cleared or no RTS instruction found!");
    }
    cpu.pc.pc = return_addr.wrapping_add(1);
}

// break

pub fn brk(cpu: &mut CPU, _: AddressingMode) {
    cpu.pc.pc += 1;
    cpu.stack.push_u16(cpu.pc.pc);
    cpu.stack.push(cpu.status.status | 0b0011_0000);
    cpu.status.set_interrupt(true);
}

pub fn rti(cpu: &mut CPU, _: AddressingMode) {
    cpu.status.status = cpu.stack.pop() & 0b1101_1111;
    cpu.pc.pc = cpu.stack.pop_u16();
}
