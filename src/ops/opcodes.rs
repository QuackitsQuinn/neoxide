
// ===================================================================================================
//  This file is generated by build.rs. DO NOT EDIT
// ===================================================================================================
//  This file contains the opcodes for the 6502 CPU, extracted from the 6502.json file in the res folder.
//  The opcodes are stored in a static array called the optable, which is indexed by the opcode byte.
//  This is **SIGNIFICANTLY** faster than a match statement, and is the reason why the optable is used.
//  The optable is generated by the build script, which is how this file is generated.
//
//  lazy_static! thank you for existing

use crate::{addressing::AddressingMode, cpu::CPU, ops::{opcode::Operation,op::{nop,undoc_nop},load_ops::*,store_ops::*,reg_ops::*,arithmatic_ops::*,branch_ops::*,stack_ops::*,status_ops::*}};

/// ADd with Carry
#[allow(non_snake_case)]
pub mod ADC {
 use super::*;

 lazy_static! {
   pub static ref IMMEDIATE: Operation = Operation::new("ADC", 0x69, adc, 2, 0, AddressingMode::Immediate);
   pub static ref ZERO_PAGE: Operation = Operation::new("ADC", 0x65, adc, 3, 0, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X: Operation = Operation::new("ADC", 0x75, adc, 4, 0, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE: Operation = Operation::new("ADC", 0x6D, adc, 4, 0, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X: Operation = Operation::new("ADC", 0x7D, adc, 4, 1, AddressingMode::AbsoluteX);
   pub static ref ABSOLUTE_Y: Operation = Operation::new("ADC", 0x79, adc, 4, 1, AddressingMode::AbsoluteY);
   pub static ref INDIRECT_X: Operation = Operation::new("ADC", 0x61, adc, 6, 0, AddressingMode::IndirectX);
   pub static ref INDIRECT_Y: Operation = Operation::new("ADC", 0x71, adc, 5, 1, AddressingMode::IndirectY);
 }
}

/// bitwise AND with accumulator
#[allow(non_snake_case)]
pub mod AND {
 use super::*;

 lazy_static! {
   pub static ref IMMEDIATE: Operation = Operation::new("AND", 0x29, and, 2, 0, AddressingMode::Immediate);
   pub static ref ZERO_PAGE: Operation = Operation::new("AND", 0x25, and, 3, 0, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X: Operation = Operation::new("AND", 0x35, and, 4, 0, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE: Operation = Operation::new("AND", 0x2D, and, 4, 0, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X: Operation = Operation::new("AND", 0x3D, and, 4, 1, AddressingMode::AbsoluteX);
   pub static ref ABSOLUTE_Y: Operation = Operation::new("AND", 0x39, and, 4, 1, AddressingMode::AbsoluteY);
   pub static ref INDIRECT_X: Operation = Operation::new("AND", 0x21, and, 6, 0, AddressingMode::IndirectX);
   pub static ref INDIRECT_Y: Operation = Operation::new("AND", 0x31, and, 5, 1, AddressingMode::IndirectY);
 }
}

/// Arithmatic Shift Left
#[allow(non_snake_case)]
pub mod ASL {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("ASL", 0x0A, asl, 2, 0, AddressingMode::Implied);
   pub static ref ZERO_PAGE: Operation = Operation::new("ASL", 0x06, asl, 5, 0, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X: Operation = Operation::new("ASL", 0x16, asl, 6, 0, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE: Operation = Operation::new("ASL", 0x0E, asl, 6, 0, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X: Operation = Operation::new("ASL", 0x1E, asl, 7, 0, AddressingMode::AbsoluteX);
 }
}

/// test BITs
#[allow(non_snake_case)]
pub mod BIT {
 use super::*;

 lazy_static! {
   pub static ref ZERO_PAGE: Operation = Operation::new("BIT", 0x24, bit, 3, 0, AddressingMode::ZeroPage);
   pub static ref ABSOLUTE: Operation = Operation::new("BIT", 0x2C, bit, 4, 0, AddressingMode::Absolute);
 }
}

/// Branch on PLus
#[allow(non_snake_case)]
pub mod BPL {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("BPL", 0x10, bpl, 2, 1, AddressingMode::Implied);
 }
}

/// Branch on MInus
#[allow(non_snake_case)]
pub mod BMI {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("BMI", 0x30, bmi, 2, 1, AddressingMode::Implied);
 }
}

/// Branch on oVerflow Clear
#[allow(non_snake_case)]
pub mod BVC {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("BVC", 0x50, bvc, 2, 1, AddressingMode::Implied);
 }
}

/// Branch on oVerflow Set
#[allow(non_snake_case)]
pub mod BVS {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("BVS", 0x70, bvs, 2, 1, AddressingMode::Implied);
 }
}

/// Branch on Carry Clear
#[allow(non_snake_case)]
pub mod BCC {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("BCC", 0x90, bcc, 2, 1, AddressingMode::Implied);
 }
}

/// Branch on Carry Set
#[allow(non_snake_case)]
pub mod BCS {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("BCS", 0xB0, bcs, 2, 1, AddressingMode::Implied);
 }
}

/// Branch on Not Equal
#[allow(non_snake_case)]
pub mod BNE {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("BNE", 0xD0, bne, 2, 1, AddressingMode::Implied);
 }
}

/// Branch on EQual
#[allow(non_snake_case)]
pub mod BEQ {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("BEQ", 0xF0, beq, 2, 1, AddressingMode::Implied);
 }
}

/// BReaK
#[allow(non_snake_case)]
pub mod BRK {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("BRK", 0x00, brk, 7, 0, AddressingMode::Implied);
 }
}

/// CoMPare accumulator
#[allow(non_snake_case)]
pub mod CMP {
 use super::*;

 lazy_static! {
   pub static ref IMMEDIATE: Operation = Operation::new("CMP", 0xC9, cmp, 2, 0, AddressingMode::Immediate);
   pub static ref ZERO_PAGE: Operation = Operation::new("CMP", 0xC5, cmp, 3, 0, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X: Operation = Operation::new("CMP", 0xD5, cmp, 4, 0, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE: Operation = Operation::new("CMP", 0xCD, cmp, 4, 0, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X: Operation = Operation::new("CMP", 0xDD, cmp, 4, 1, AddressingMode::AbsoluteX);
   pub static ref ABSOLUTE_Y: Operation = Operation::new("CMP", 0xD9, cmp, 4, 1, AddressingMode::AbsoluteY);
   pub static ref INDIRECT_X: Operation = Operation::new("CMP", 0xC1, cmp, 6, 0, AddressingMode::IndirectX);
   pub static ref INDIRECT_Y: Operation = Operation::new("CMP", 0xD1, cmp, 5, 1, AddressingMode::IndirectY);
 }
}

/// ComPare X register
#[allow(non_snake_case)]
pub mod CPX {
 use super::*;

 lazy_static! {
   pub static ref IMMEDIATE: Operation = Operation::new("CPX", 0xE0, cpx, 2, 0, AddressingMode::Immediate);
   pub static ref ZERO_PAGE: Operation = Operation::new("CPX", 0xE4, cpx, 3, 0, AddressingMode::ZeroPage);
   pub static ref ABSOLUTE: Operation = Operation::new("CPX", 0xEC, cpx, 4, 0, AddressingMode::Absolute);
 }
}

/// ComPare Y register
#[allow(non_snake_case)]
pub mod CPY {
 use super::*;

 lazy_static! {
   pub static ref IMMEDIATE: Operation = Operation::new("CPY", 0xC0, cpy, 2, 0, AddressingMode::Immediate);
   pub static ref ZERO_PAGE: Operation = Operation::new("CPY", 0xC4, cpy, 3, 0, AddressingMode::ZeroPage);
   pub static ref ABSOLUTE: Operation = Operation::new("CPY", 0xCC, cpy, 4, 0, AddressingMode::Absolute);
 }
}

/// DECrement memory
#[allow(non_snake_case)]
pub mod DEC {
 use super::*;

 lazy_static! {
   pub static ref ZERO_PAGE: Operation = Operation::new("DEC", 0xC6, dec, 5, 0, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X: Operation = Operation::new("DEC", 0xD6, dec, 6, 0, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE: Operation = Operation::new("DEC", 0xCE, dec, 6, 0, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X: Operation = Operation::new("DEC", 0xDE, dec, 7, 0, AddressingMode::AbsoluteX);
 }
}

/// bitwise Exclusive OR
#[allow(non_snake_case)]
pub mod EOR {
 use super::*;

 lazy_static! {
   pub static ref IMMEDIATE: Operation = Operation::new("EOR", 0x49, eor, 2, 0, AddressingMode::Immediate);
   pub static ref ZERO_PAGE: Operation = Operation::new("EOR", 0x45, eor, 3, 0, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X: Operation = Operation::new("EOR", 0x55, eor, 4, 0, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE: Operation = Operation::new("EOR", 0x4D, eor, 4, 0, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X: Operation = Operation::new("EOR", 0x5D, eor, 4, 1, AddressingMode::AbsoluteX);
   pub static ref ABSOLUTE_Y: Operation = Operation::new("EOR", 0x59, eor, 4, 1, AddressingMode::AbsoluteY);
   pub static ref INDIRECT_X: Operation = Operation::new("EOR", 0x41, eor, 6, 0, AddressingMode::IndirectX);
   pub static ref INDIRECT_Y: Operation = Operation::new("EOR", 0x51, eor, 5, 1, AddressingMode::IndirectY);
 }
}

/// CLear Carry
#[allow(non_snake_case)]
pub mod CLC {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("CLC", 0x18, clc, 2, 0, AddressingMode::Implied);
 }
}

/// SEt Carry
#[allow(non_snake_case)]
pub mod SEC {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("SEC", 0x38, sec, 2, 0, AddressingMode::Implied);
 }
}

/// CLear Interrupt
#[allow(non_snake_case)]
pub mod CLI {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("CLI", 0x58, cli, 2, 0, AddressingMode::Implied);
 }
}

/// SEt Interrupt
#[allow(non_snake_case)]
pub mod SEI {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("SEI", 0x78, sei, 2, 0, AddressingMode::Implied);
 }
}

/// CLear oVerflow
#[allow(non_snake_case)]
pub mod CLV {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("CLV", 0xB8, clv, 2, 0, AddressingMode::Implied);
 }
}

/// CLear Decimal
#[allow(non_snake_case)]
pub mod CLD {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("CLD", 0xD8, cld, 2, 0, AddressingMode::Implied);
 }
}

/// SEt Decimal
#[allow(non_snake_case)]
pub mod SED {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("SED", 0xF8, sed, 2, 0, AddressingMode::Implied);
 }
}

/// INCrement memory
#[allow(non_snake_case)]
pub mod INC {
 use super::*;

 lazy_static! {
   pub static ref ZERO_PAGE: Operation = Operation::new("INC", 0xE6, inc, 5, 0, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X: Operation = Operation::new("INC", 0xF6, inc, 6, 0, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE: Operation = Operation::new("INC", 0xEE, inc, 6, 0, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X: Operation = Operation::new("INC", 0xFE, inc, 7, 0, AddressingMode::AbsoluteX);
 }
}

/// JuMP
#[allow(non_snake_case)]
pub mod JMP {
 use super::*;

 lazy_static! {
   pub static ref ABSOLUTE: Operation = Operation::new("JMP", 0x4C, jmp, 3, 0, AddressingMode::Absolute);
   pub static ref INDIRECT: Operation = Operation::new("JMP", 0x6C, jmp, 5, 0, AddressingMode::Indirect);
 }
}

/// Jump to SubRoutine
#[allow(non_snake_case)]
pub mod JSR {
 use super::*;

 lazy_static! {
   pub static ref ABSOLUTE: Operation = Operation::new("JSR", 0x20, jsr, 6, 0, AddressingMode::Absolute);
 }
}

/// LoaD Accumulator
#[allow(non_snake_case)]
pub mod LDA {
 use super::*;

 lazy_static! {
   pub static ref IMMEDIATE: Operation = Operation::new("LDA", 0xA9, lda, 2, 0, AddressingMode::Immediate);
   pub static ref ZERO_PAGE: Operation = Operation::new("LDA", 0xA5, lda, 3, 0, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X: Operation = Operation::new("LDA", 0xB5, lda, 4, 0, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE: Operation = Operation::new("LDA", 0xAD, lda, 4, 0, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X: Operation = Operation::new("LDA", 0xBD, lda, 4, 1, AddressingMode::AbsoluteX);
   pub static ref ABSOLUTE_Y: Operation = Operation::new("LDA", 0xB9, lda, 4, 1, AddressingMode::AbsoluteY);
   pub static ref INDIRECT_X: Operation = Operation::new("LDA", 0xA1, lda, 6, 0, AddressingMode::IndirectX);
   pub static ref INDIRECT_Y: Operation = Operation::new("LDA", 0xB1, lda, 5, 1, AddressingMode::IndirectY);
 }
}

/// LoaD X register
#[allow(non_snake_case)]
pub mod LDX {
 use super::*;

 lazy_static! {
   pub static ref IMMEDIATE: Operation = Operation::new("LDX", 0xA2, ldx, 2, 0, AddressingMode::Immediate);
   pub static ref ZERO_PAGE: Operation = Operation::new("LDX", 0xA6, ldx, 3, 0, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_Y: Operation = Operation::new("LDX", 0xB6, ldx, 4, 0, AddressingMode::ZeroPageY);
   pub static ref ABSOLUTE: Operation = Operation::new("LDX", 0xAE, ldx, 4, 0, AddressingMode::Absolute);
   pub static ref ABSOLUTE_Y: Operation = Operation::new("LDX", 0xBE, ldx, 4, 1, AddressingMode::AbsoluteY);
 }
}

/// LoaD Y register
#[allow(non_snake_case)]
pub mod LDY {
 use super::*;

 lazy_static! {
   pub static ref IMMEDIATE: Operation = Operation::new("LDY", 0xA0, ldy, 2, 0, AddressingMode::Immediate);
   pub static ref ZERO_PAGE: Operation = Operation::new("LDY", 0xA4, ldy, 3, 0, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X: Operation = Operation::new("LDY", 0xB4, ldy, 4, 0, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE: Operation = Operation::new("LDY", 0xAC, ldy, 4, 0, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X: Operation = Operation::new("LDY", 0xBC, ldy, 4, 1, AddressingMode::AbsoluteX);
 }
}

/// Logical Shift Right
#[allow(non_snake_case)]
pub mod LSR {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("LSR", 0x4A, lsr, 2, 0, AddressingMode::Implied);
   pub static ref ZERO_PAGE: Operation = Operation::new("LSR", 0x46, lsr, 5, 0, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X: Operation = Operation::new("LSR", 0x56, lsr, 6, 0, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE: Operation = Operation::new("LSR", 0x4E, lsr, 6, 0, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X: Operation = Operation::new("LSR", 0x5E, lsr, 7, 0, AddressingMode::AbsoluteX);
 }
}

/// No OPeration
#[allow(non_snake_case)]
pub mod NOP {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("NOP", 0xEA, nop, 2, 0, AddressingMode::Implied);
 }
}

/// bitwise OR with Accumulator
#[allow(non_snake_case)]
pub mod ORA {
 use super::*;

 lazy_static! {
   pub static ref IMMEDIATE: Operation = Operation::new("ORA", 0x09, ora, 2, 0, AddressingMode::Immediate);
   pub static ref ZERO_PAGE: Operation = Operation::new("ORA", 0x05, ora, 3, 0, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X: Operation = Operation::new("ORA", 0x15, ora, 4, 0, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE: Operation = Operation::new("ORA", 0x0D, ora, 4, 0, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X: Operation = Operation::new("ORA", 0x1D, ora, 4, 1, AddressingMode::AbsoluteX);
   pub static ref ABSOLUTE_Y: Operation = Operation::new("ORA", 0x19, ora, 4, 1, AddressingMode::AbsoluteY);
   pub static ref INDIRECT_X: Operation = Operation::new("ORA", 0x01, ora, 6, 0, AddressingMode::IndirectX);
   pub static ref INDIRECT_Y: Operation = Operation::new("ORA", 0x11, ora, 5, 1, AddressingMode::IndirectY);
 }
}

/// Transfer Accumulator to X
#[allow(non_snake_case)]
pub mod TAX {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("TAX", 0xAA, tax, 2, 0, AddressingMode::Implied);
 }
}

/// Transfer X to Accumulator
#[allow(non_snake_case)]
pub mod TXA {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("TXA", 0x8A, txa, 2, 0, AddressingMode::Implied);
 }
}

/// DEcrement X
#[allow(non_snake_case)]
pub mod DEX {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("DEX", 0xCA, dex, 2, 0, AddressingMode::Implied);
 }
}

/// INcrement X
#[allow(non_snake_case)]
pub mod INX {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("INX", 0xE8, inx, 2, 0, AddressingMode::Implied);
 }
}

/// Transfer Accumulator to Y
#[allow(non_snake_case)]
pub mod TAY {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("TAY", 0xA8, tay, 2, 0, AddressingMode::Implied);
 }
}

/// Transfer Y to Accumulator
#[allow(non_snake_case)]
pub mod TYA {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("TYA", 0x98, tya, 2, 0, AddressingMode::Implied);
 }
}

/// DEcrement Y
#[allow(non_snake_case)]
pub mod DEY {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("DEY", 0x88, dey, 2, 0, AddressingMode::Implied);
 }
}

/// INcrement Y
#[allow(non_snake_case)]
pub mod INY {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("INY", 0xC8, iny, 2, 0, AddressingMode::Implied);
 }
}

/// ROtate Left
#[allow(non_snake_case)]
pub mod ROL {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("ROL", 0x2A, rol, 2, 0, AddressingMode::Implied);
   pub static ref ZERO_PAGE: Operation = Operation::new("ROL", 0x26, rol, 5, 0, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X: Operation = Operation::new("ROL", 0x36, rol, 6, 0, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE: Operation = Operation::new("ROL", 0x2E, rol, 6, 0, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X: Operation = Operation::new("ROL", 0x3E, rol, 7, 0, AddressingMode::AbsoluteX);
 }
}

/// ROtate Right
#[allow(non_snake_case)]
pub mod ROR {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("ROR", 0x6A, ror, 2, 0, AddressingMode::Implied);
   pub static ref ZERO_PAGE: Operation = Operation::new("ROR", 0x66, ror, 5, 0, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X: Operation = Operation::new("ROR", 0x76, ror, 6, 0, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE: Operation = Operation::new("ROR", 0x6E, ror, 6, 0, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X: Operation = Operation::new("ROR", 0x7E, ror, 7, 0, AddressingMode::AbsoluteX);
 }
}

/// ReTurn from Interrupt
#[allow(non_snake_case)]
pub mod RTI {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("RTI", 0x40, rti, 6, 0, AddressingMode::Implied);
 }
}

/// ReTurn from Subroutine
#[allow(non_snake_case)]
pub mod RTS {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("RTS", 0x60, rts, 6, 0, AddressingMode::Implied);
 }
}

/// SuBtract with Carry
#[allow(non_snake_case)]
pub mod SBC {
 use super::*;

 lazy_static! {
   pub static ref IMMEDIATE: Operation = Operation::new("SBC", 0xE9, sbc, 2, 0, AddressingMode::Immediate);
   pub static ref ZERO_PAGE: Operation = Operation::new("SBC", 0xE5, sbc, 3, 0, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X: Operation = Operation::new("SBC", 0xF5, sbc, 4, 0, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE: Operation = Operation::new("SBC", 0xED, sbc, 4, 0, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X: Operation = Operation::new("SBC", 0xFD, sbc, 4, 1, AddressingMode::AbsoluteX);
   pub static ref ABSOLUTE_Y: Operation = Operation::new("SBC", 0xF9, sbc, 4, 1, AddressingMode::AbsoluteY);
   pub static ref INDIRECT_X: Operation = Operation::new("SBC", 0xE1, sbc, 6, 0, AddressingMode::IndirectX);
   pub static ref INDIRECT_Y: Operation = Operation::new("SBC", 0xF1, sbc, 5, 1, AddressingMode::IndirectY);
 }
}

/// STore Accumulator
#[allow(non_snake_case)]
pub mod STA {
 use super::*;

 lazy_static! {
   pub static ref ZERO_PAGE: Operation = Operation::new("STA", 0x85, sta, 3, 0, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X: Operation = Operation::new("STA", 0x95, sta, 4, 0, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE: Operation = Operation::new("STA", 0x8D, sta, 4, 0, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X: Operation = Operation::new("STA", 0x9D, sta, 5, 0, AddressingMode::AbsoluteX);
   pub static ref ABSOLUTE_Y: Operation = Operation::new("STA", 0x99, sta, 5, 0, AddressingMode::AbsoluteY);
   pub static ref INDIRECT_X: Operation = Operation::new("STA", 0x81, sta, 6, 0, AddressingMode::IndirectX);
   pub static ref INDIRECT_Y: Operation = Operation::new("STA", 0x91, sta, 6, 0, AddressingMode::IndirectY);
 }
}

/// Transfer X to Stack ptr
#[allow(non_snake_case)]
pub mod TXS {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("TXS", 0x9A, txs, 2, 0, AddressingMode::Implied);
 }
}

/// Transfer Stack ptr to X
#[allow(non_snake_case)]
pub mod TSX {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("TSX", 0xBA, tsx, 2, 0, AddressingMode::Implied);
 }
}

/// PusH Accumulator
#[allow(non_snake_case)]
pub mod PHA {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("PHA", 0x48, pha, 3, 0, AddressingMode::Implied);
 }
}

/// PuLl Accumulator
#[allow(non_snake_case)]
pub mod PLA {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("PLA", 0x68, pla, 4, 0, AddressingMode::Implied);
 }
}

/// PusH Processor status
#[allow(non_snake_case)]
pub mod PHP {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("PHP", 0x08, php, 3, 0, AddressingMode::Implied);
 }
}

/// PuLl Processor status
#[allow(non_snake_case)]
pub mod PLP {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("PLP", 0x28, plp, 4, 0, AddressingMode::Implied);
 }
}

/// STore X register
#[allow(non_snake_case)]
pub mod STX {
 use super::*;

 lazy_static! {
   pub static ref ZERO_PAGE: Operation = Operation::new("STX", 0x86, stx, 3, 0, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_Y: Operation = Operation::new("STX", 0x96, stx, 4, 0, AddressingMode::ZeroPageY);
   pub static ref ABSOLUTE: Operation = Operation::new("STX", 0x8E, stx, 4, 0, AddressingMode::Absolute);
 }
}

/// STore Y register
#[allow(non_snake_case)]
pub mod STY {
 use super::*;

 lazy_static! {
   pub static ref ZERO_PAGE: Operation = Operation::new("STY", 0x84, sty, 3, 0, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X: Operation = Operation::new("STY", 0x94, sty, 4, 0, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE: Operation = Operation::new("STY", 0x8C, sty, 4, 0, AddressingMode::Absolute);
 }
}

/// Undocumented No-Op
#[allow(non_snake_case)]
pub mod UNDOC_NOP {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: Operation = Operation::new("UNDOC_NOP", 0xEA, undoc_nop, 2, 0, AddressingMode::Implied);
 }
}

lazy_static! {

 
/// This is the optable, which contains all of the opcodes for the 6502 CPU.
/// The order of the ops is **EXTREMELY** important, as the index is the opcode byte.
/// Any opcode marked as UNDOC_NOP is an undocumented opcode, and will be logged when executed, but will not do anything.
/// The optable is like a huge match statement, but is **SIGNIFICANTLY** faster because it is a static array.
 
    pub static ref OPTABLE: [Operation; 255] = [
        *BRK::IMPLIED,
        *ORA::INDIRECT_X,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *ORA::ZERO_PAGE,
        *ASL::ZERO_PAGE,
        *UNDOC_NOP::IMPLIED,
        *PHP::IMPLIED,
        *ORA::IMMEDIATE,
        *ASL::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *ORA::ABSOLUTE,
        *ASL::ABSOLUTE,
        *UNDOC_NOP::IMPLIED,
        *BPL::IMPLIED,
        *ORA::INDIRECT_Y,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *ORA::ZERO_PAGE_X,
        *ASL::ZERO_PAGE_X,
        *UNDOC_NOP::IMPLIED,
        *CLC::IMPLIED,
        *ORA::ABSOLUTE_Y,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *ORA::ABSOLUTE_X,
        *ASL::ABSOLUTE_X,
        *UNDOC_NOP::IMPLIED,
        *JSR::ABSOLUTE,
        *AND::INDIRECT_X,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *BIT::ZERO_PAGE,
        *AND::ZERO_PAGE,
        *ROL::ZERO_PAGE,
        *UNDOC_NOP::IMPLIED,
        *PLP::IMPLIED,
        *AND::IMMEDIATE,
        *ROL::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *BIT::ABSOLUTE,
        *AND::ABSOLUTE,
        *ROL::ABSOLUTE,
        *UNDOC_NOP::IMPLIED,
        *BMI::IMPLIED,
        *AND::INDIRECT_Y,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *AND::ZERO_PAGE_X,
        *ROL::ZERO_PAGE_X,
        *UNDOC_NOP::IMPLIED,
        *SEC::IMPLIED,
        *AND::ABSOLUTE_Y,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *AND::ABSOLUTE_X,
        *ROL::ABSOLUTE_X,
        *UNDOC_NOP::IMPLIED,
        *RTI::IMPLIED,
        *EOR::INDIRECT_X,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *EOR::ZERO_PAGE,
        *LSR::ZERO_PAGE,
        *UNDOC_NOP::IMPLIED,
        *PHA::IMPLIED,
        *EOR::IMMEDIATE,
        *LSR::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *JMP::ABSOLUTE,
        *EOR::ABSOLUTE,
        *LSR::ABSOLUTE,
        *UNDOC_NOP::IMPLIED,
        *BVC::IMPLIED,
        *EOR::INDIRECT_Y,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *EOR::ZERO_PAGE_X,
        *LSR::ZERO_PAGE_X,
        *UNDOC_NOP::IMPLIED,
        *CLI::IMPLIED,
        *EOR::ABSOLUTE_Y,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *EOR::ABSOLUTE_X,
        *LSR::ABSOLUTE_X,
        *UNDOC_NOP::IMPLIED,
        *RTS::IMPLIED,
        *ADC::INDIRECT_X,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *ADC::ZERO_PAGE,
        *ROR::ZERO_PAGE,
        *UNDOC_NOP::IMPLIED,
        *PLA::IMPLIED,
        *ADC::IMMEDIATE,
        *ROR::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *JMP::INDIRECT,
        *ADC::ABSOLUTE,
        *ROR::ABSOLUTE,
        *UNDOC_NOP::IMPLIED,
        *BVS::IMPLIED,
        *ADC::INDIRECT_Y,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *ADC::ZERO_PAGE_X,
        *ROR::ZERO_PAGE_X,
        *UNDOC_NOP::IMPLIED,
        *SEI::IMPLIED,
        *ADC::ABSOLUTE_Y,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *ADC::ABSOLUTE_X,
        *ROR::ABSOLUTE_X,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *STA::INDIRECT_X,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *STY::ZERO_PAGE,
        *STA::ZERO_PAGE,
        *STX::ZERO_PAGE,
        *UNDOC_NOP::IMPLIED,
        *DEY::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *TXA::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *STY::ABSOLUTE,
        *STA::ABSOLUTE,
        *STX::ABSOLUTE,
        *UNDOC_NOP::IMPLIED,
        *BCC::IMPLIED,
        *STA::INDIRECT_Y,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *STY::ZERO_PAGE_X,
        *STA::ZERO_PAGE_X,
        *STX::ZERO_PAGE_Y,
        *UNDOC_NOP::IMPLIED,
        *TYA::IMPLIED,
        *STA::ABSOLUTE_Y,
        *TXS::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *STA::ABSOLUTE_X,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *LDY::IMMEDIATE,
        *LDA::INDIRECT_X,
        *LDX::IMMEDIATE,
        *UNDOC_NOP::IMPLIED,
        *LDY::ZERO_PAGE,
        *LDA::ZERO_PAGE,
        *LDX::ZERO_PAGE,
        *UNDOC_NOP::IMPLIED,
        *TAY::IMPLIED,
        *LDA::IMMEDIATE,
        *TAX::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *LDY::ABSOLUTE,
        *LDA::ABSOLUTE,
        *LDX::ABSOLUTE,
        *UNDOC_NOP::IMPLIED,
        *BCS::IMPLIED,
        *LDA::INDIRECT_Y,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *LDY::ZERO_PAGE_X,
        *LDA::ZERO_PAGE_X,
        *LDX::ZERO_PAGE_Y,
        *UNDOC_NOP::IMPLIED,
        *CLV::IMPLIED,
        *LDA::ABSOLUTE_Y,
        *TSX::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *LDY::ABSOLUTE_X,
        *LDA::ABSOLUTE_X,
        *LDX::ABSOLUTE_Y,
        *UNDOC_NOP::IMPLIED,
        *CPY::IMMEDIATE,
        *CMP::INDIRECT_X,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *CPY::ZERO_PAGE,
        *CMP::ZERO_PAGE,
        *DEC::ZERO_PAGE,
        *UNDOC_NOP::IMPLIED,
        *INY::IMPLIED,
        *CMP::IMMEDIATE,
        *DEX::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *CPY::ABSOLUTE,
        *CMP::ABSOLUTE,
        *DEC::ABSOLUTE,
        *UNDOC_NOP::IMPLIED,
        *BNE::IMPLIED,
        *CMP::INDIRECT_Y,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *CMP::ZERO_PAGE_X,
        *DEC::ZERO_PAGE_X,
        *UNDOC_NOP::IMPLIED,
        *CLD::IMPLIED,
        *CMP::ABSOLUTE_Y,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *CMP::ABSOLUTE_X,
        *DEC::ABSOLUTE_X,
        *UNDOC_NOP::IMPLIED,
        *CPX::IMMEDIATE,
        *SBC::INDIRECT_X,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *CPX::ZERO_PAGE,
        *SBC::ZERO_PAGE,
        *INC::ZERO_PAGE,
        *UNDOC_NOP::IMPLIED,
        *INX::IMPLIED,
        *SBC::IMMEDIATE,
        *NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *CPX::ABSOLUTE,
        *SBC::ABSOLUTE,
        *INC::ABSOLUTE,
        *UNDOC_NOP::IMPLIED,
        *BEQ::IMPLIED,
        *SBC::INDIRECT_Y,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *SBC::ZERO_PAGE_X,
        *INC::ZERO_PAGE_X,
        *UNDOC_NOP::IMPLIED,
        *SED::IMPLIED,
        *SBC::ABSOLUTE_Y,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *UNDOC_NOP::IMPLIED,
        *SBC::ABSOLUTE_X,
        *INC::ABSOLUTE_X,
    ];
}