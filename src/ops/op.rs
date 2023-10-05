use crate::{cpu::CPU, addressing::AddressingMode};

use super::load_ops::{lda, ldx, ldy};
/// Delegates the execution of the next operation to the appropriate function.  
/// This function is here because a 255 line match statement is not very readable to be in cpu.rs
pub fn exec_op(cpu: &mut CPU) {
    let op = cpu.read_opbyte();

    match op {
        0xEA => nop(cpu),
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
        0xA0 => ldy::ldy_im(cpu),
        0xA4 => ldy::ldy_zp(cpu),
        0xB4 => ldy::ldy_zp_x(cpu),
        0xAC => ldy::ldy_abs(cpu),
        0xBC => ldy::ldy_abs_x(cpu),
        _ => todo!("Implement op: {:X}", op),
    }
}
/// No op - does nothing
fn nop(_cpu: &mut CPU) {}

pub fn read_u16(cpu: &mut CPU) -> u16 {
    let addr1 = cpu.read_opbyte();
    let addr2 = cpu.read_opbyte();
    ((addr2 as u16) << 8) | addr1 as u16
}
