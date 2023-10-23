use crate::{addressing::AddressingMode, cpu::CPU};

use super::{
    branch_ops::{bcc, bcs, beq, bmi, bne, bpl, bvc, bvs, jmp},
    load_ops::{lda, ldx, ldy},
    reg_ops::{dex, dey, inx, iny, tax, tay, tsx, txa, txs, tya},
    stack_ops::{pha, php, pla, plp},
    status_ops::{clc, cli, clv, sec, sei},
    store_ops::{sta, stx, sty},
};
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
        // STORE OPS
        // STA
        0x85 => sta(cpu, AddressingMode::ZeroPage),
        0x95 => sta(cpu, AddressingMode::ZeroPageX),
        0x8D => sta(cpu, AddressingMode::Absolute),
        0x9D => sta(cpu, AddressingMode::AbsoluteX),
        0x81 => sta(cpu, AddressingMode::IndirectX),
        0x91 => sta(cpu, AddressingMode::IndirectY),
        // STX
        0x86 => stx(cpu, AddressingMode::ZeroPage),
        0x96 => stx(cpu, AddressingMode::ZeroPageY),
        0x8E => stx(cpu, AddressingMode::Absolute),
        // STY
        0x84 => sty(cpu, AddressingMode::ZeroPage),
        0x94 => sty(cpu, AddressingMode::ZeroPageX),
        0x8C => sty(cpu, AddressingMode::Absolute),
        // TRANSFER OPS
        0xAA => tax(cpu),
        0xA8 => tay(cpu),
        0x8A => txa(cpu),
        0x98 => tya(cpu),
        0xBA => tsx(cpu),
        0x9A => txs(cpu),
        // REG OPS
        0xE8 => inx(cpu),
        0xC8 => iny(cpu),
        0xCA => dex(cpu),
        0x88 => dey(cpu),
        // STACK OPS
        0x48 => pha(cpu),
        0x68 => pla(cpu),
        0x08 => php(cpu),
        0x28 => plp(cpu),
        // STATUS OPS
        0x18 => clc(cpu),
        0x38 => sec(cpu),
        0x58 => cli(cpu),
        0x78 => sei(cpu),
        0xB8 => clv(cpu),
        // BRANCH OPS
        0x4C => jmp(cpu, AddressingMode::Absolute),
        0x6C => jmp(cpu, AddressingMode::Indirect),
        0xD0 => bne(cpu),
        0xF0 => beq(cpu),
        0x10 => bpl(cpu),
        0x30 => bmi(cpu),
        0x50 => bvc(cpu),
        0x70 => bvs(cpu),
        0x90 => bcc(cpu),
        0xB0 => bcs(cpu),

        _ => todo!("Unimplemented opcode: {:#X}", op),
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
