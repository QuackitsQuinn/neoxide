use crate::{cpu::CPU, addressing::AddressingMode};

use super::{load_ops::{lda, ldx, ldy}, trans_ops::{tax, tay, txa, tya, tsx, txs}, stack_ops::{pha, pla, php, plp}};
/// Delegates the execution of the next operation to the appropriate function.  
/// This function is here because a 255 line match statement is not very readable to be in cpu.rs
pub fn exec_op(cpu: &mut CPU) {
    let op = cpu.read_opbyte();

    match op {
        0xEA => nop(cpu),
        // LOAD OPS
        // LDA
        0xA9 => lda(cpu, AddressingMode::Immediate),
        0xA5 => lda(cpu, AddressingMode::ZeroPage),
        0xB5 => lda(cpu, AddressingMode::ZeroPageX),
        0xAD => lda(cpu, AddressingMode::Absolute),
        0xBD => lda(cpu, AddressingMode::AbsoluteX),
        0xB9 => lda(cpu, AddressingMode::AbsoluteY),
        0xA1 => lda(cpu, AddressingMode::IndirectX),
        0xB1 => lda(cpu, AddressingMode::IndirectY),
        // LDX
        0xA2 => ldx(cpu, AddressingMode::Immediate),
        0xA6 => ldx(cpu, AddressingMode::ZeroPage),
        0xB6 => ldx(cpu, AddressingMode::ZeroPageY),
        0xAE => ldx(cpu, AddressingMode::Absolute),
        0xBE => ldx(cpu, AddressingMode::AbsoluteY),
        // LDY
        0xA0 => ldy(cpu, AddressingMode::Immediate),
        0xA4 => ldy(cpu, AddressingMode::ZeroPage),
        0xB4 => ldy(cpu, AddressingMode::ZeroPageX),
        0xAC => ldy(cpu, AddressingMode::Absolute),
        0xBC => ldy(cpu, AddressingMode::AbsoluteX),
        // TRANSFER OPS
        0xAA => tax(cpu),
        0xA8 => tay(cpu),
        0x8A => txa(cpu),
        0x98 => tya(cpu),
        0xBA => tsx(cpu),
        0x9A => txs(cpu),
        // STACK OPS
        0x48 => pha(cpu),
        0x68 => pla(cpu),
        0x08 => php(cpu),
        0x28 => plp(cpu),
        _ => panic!("Unimplemented opcode: {:#X}", op),
    }
}
/// No op - does nothing
fn nop(_cpu: &mut CPU) {}

/// Checks the data for zero and negative flags and sets them accordingly
pub fn check_flags(cpu: &mut CPU, data: u8) {
    if data == 0 {
        cpu.status.set_zero(true);
    } else {
        cpu.status.set_zero(false);
    }

    if data & 0x80 == 0x80 {
        cpu.status.set_negative(true);
    } else {
        cpu.status.set_negative(false);
    }
}