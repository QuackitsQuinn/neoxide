use crate::{cpu::CPU, addressing::AddressingMode};



pub fn jmp(cpu: &mut CPU, mode: AddressingMode) {
    let addr = cpu.get_addr(mode);
    cpu.pc.pc = addr;
}