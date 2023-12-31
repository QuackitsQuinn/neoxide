
#![cfg_attr(rustfmt, rustfmt_skip)]
// ===================================================================================================
//  This file is generated by build.rs. DO NOT EDIT
// ===================================================================================================
//  This file contains the opcodes for the 6502 CPU, extracted from the 6502.json file in the res folder.
//  The opcodes are stored in a static array called the optable, which is indexed by the opcode byte.
//  This is **SIGNIFICANTLY** faster than a match statement, and is the reason why the optable is used.
//  The optable is generated by the build script, which is how this file is generated.
//
//  lazy_static! thank you for existing

use crate::{addressing::AddressingMode, ops::{opcode::{OpCode,Operation},op::{nop,undoc_nop},load_ops::*,store_ops::*,reg_ops::*,arithmatic_ops::*,branch_ops::*,stack_ops::*,status_ops::*}};

/// ADd with Carry
#[allow(non_snake_case)]
pub mod adc {
 use super::*;

 lazy_static! {
   pub static ref IMMEDIATE   : OpCode = OpCode::new("ADC", "arithmatic", 0x69, adc, 2, 0, 2, AddressingMode::Immediate);
   pub static ref ZERO_PAGE   : OpCode = OpCode::new("ADC", "arithmatic", 0x65, adc, 3, 0, 2, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X : OpCode = OpCode::new("ADC", "arithmatic", 0x75, adc, 4, 0, 2, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE    : OpCode = OpCode::new("ADC", "arithmatic", 0x6D, adc, 4, 0, 3, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X  : OpCode = OpCode::new("ADC", "arithmatic", 0x7D, adc, 4, 1, 3, AddressingMode::AbsoluteX);
   pub static ref ABSOLUTE_Y  : OpCode = OpCode::new("ADC", "arithmatic", 0x79, adc, 4, 1, 3, AddressingMode::AbsoluteY);
   pub static ref INDIRECT_X  : OpCode = OpCode::new("ADC", "arithmatic", 0x61, adc, 6, 0, 2, AddressingMode::IndirectX);
   pub static ref INDIRECT_Y  : OpCode = OpCode::new("ADC", "arithmatic", 0x71, adc, 5, 1, 2, AddressingMode::IndirectY);
 
   pub static ref ADC: Operation = Operation::new("ADC", "arithmatic", vec![*IMMEDIATE,*ZERO_PAGE,*ZERO_PAGE_X,*ABSOLUTE,*ABSOLUTE_X,*ABSOLUTE_Y,*INDIRECT_X,*INDIRECT_Y,]);

 }
}

/// bitwise AND with accumulator
#[allow(non_snake_case)]
pub mod and {
 use super::*;

 lazy_static! {
   pub static ref IMMEDIATE   : OpCode = OpCode::new("AND", "arithmatic", 0x29, and, 2, 0, 2, AddressingMode::Immediate);
   pub static ref ZERO_PAGE   : OpCode = OpCode::new("AND", "arithmatic", 0x25, and, 3, 0, 2, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X : OpCode = OpCode::new("AND", "arithmatic", 0x35, and, 4, 0, 2, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE    : OpCode = OpCode::new("AND", "arithmatic", 0x2D, and, 4, 0, 3, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X  : OpCode = OpCode::new("AND", "arithmatic", 0x3D, and, 4, 1, 3, AddressingMode::AbsoluteX);
   pub static ref ABSOLUTE_Y  : OpCode = OpCode::new("AND", "arithmatic", 0x39, and, 4, 1, 3, AddressingMode::AbsoluteY);
   pub static ref INDIRECT_X  : OpCode = OpCode::new("AND", "arithmatic", 0x21, and, 6, 0, 2, AddressingMode::IndirectX);
   pub static ref INDIRECT_Y  : OpCode = OpCode::new("AND", "arithmatic", 0x31, and, 5, 1, 2, AddressingMode::IndirectY);
 
   pub static ref AND: Operation = Operation::new("AND", "arithmatic", vec![*IMMEDIATE,*ZERO_PAGE,*ZERO_PAGE_X,*ABSOLUTE,*ABSOLUTE_X,*ABSOLUTE_Y,*INDIRECT_X,*INDIRECT_Y,]);

 }
}

/// Arithmatic Shift Left
#[allow(non_snake_case)]
pub mod asl {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("ASL", "arithmatic", 0x0A, asl, 2, 0, 1, AddressingMode::Implied);
   pub static ref ZERO_PAGE   : OpCode = OpCode::new("ASL", "arithmatic", 0x06, asl, 5, 0, 2, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X : OpCode = OpCode::new("ASL", "arithmatic", 0x16, asl, 6, 0, 2, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE    : OpCode = OpCode::new("ASL", "arithmatic", 0x0E, asl, 6, 0, 3, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X  : OpCode = OpCode::new("ASL", "arithmatic", 0x1E, asl, 7, 0, 3, AddressingMode::AbsoluteX);
 
   pub static ref ASL: Operation = Operation::new("ASL", "arithmatic", vec![*IMPLIED,*ZERO_PAGE,*ZERO_PAGE_X,*ABSOLUTE,*ABSOLUTE_X,]);

 }
}

/// test BITs
#[allow(non_snake_case)]
pub mod bit {
 use super::*;

 lazy_static! {
   pub static ref ZERO_PAGE   : OpCode = OpCode::new("BIT", "logical", 0x24, bit, 3, 0, 2, AddressingMode::ZeroPage);
   pub static ref ABSOLUTE    : OpCode = OpCode::new("BIT", "logical", 0x2C, bit, 4, 0, 3, AddressingMode::Absolute);
 
   pub static ref BIT: Operation = Operation::new("BIT", "logical", vec![*ZERO_PAGE,*ABSOLUTE,]);

 }
}

/// Branch on PLus
#[allow(non_snake_case)]
pub mod bpl {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("BPL", "branch", 0x10, bpl, 2, 1, 2, AddressingMode::Implied);
 
   pub static ref BPL: Operation = Operation::new("BPL", "branch", vec![*IMPLIED,]);

 }
}

/// Branch on MInus
#[allow(non_snake_case)]
pub mod bmi {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("BMI", "branch", 0x30, bmi, 2, 1, 2, AddressingMode::Implied);
 
   pub static ref BMI: Operation = Operation::new("BMI", "branch", vec![*IMPLIED,]);

 }
}

/// Branch on oVerflow Clear
#[allow(non_snake_case)]
pub mod bvc {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("BVC", "branch", 0x50, bvc, 2, 1, 2, AddressingMode::Implied);
 
   pub static ref BVC: Operation = Operation::new("BVC", "branch", vec![*IMPLIED,]);

 }
}

/// Branch on oVerflow Set
#[allow(non_snake_case)]
pub mod bvs {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("BVS", "branch", 0x70, bvs, 2, 1, 2, AddressingMode::Implied);
 
   pub static ref BVS: Operation = Operation::new("BVS", "branch", vec![*IMPLIED,]);

 }
}

/// Branch on Carry Clear
#[allow(non_snake_case)]
pub mod bcc {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("BCC", "branch", 0x90, bcc, 2, 1, 2, AddressingMode::Implied);
 
   pub static ref BCC: Operation = Operation::new("BCC", "branch", vec![*IMPLIED,]);

 }
}

/// Branch on Carry Set
#[allow(non_snake_case)]
pub mod bcs {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("BCS", "branch", 0xB0, bcs, 2, 1, 2, AddressingMode::Implied);
 
   pub static ref BCS: Operation = Operation::new("BCS", "branch", vec![*IMPLIED,]);

 }
}

/// Branch on Not Equal
#[allow(non_snake_case)]
pub mod bne {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("BNE", "branch", 0xD0, bne, 2, 1, 2, AddressingMode::Implied);
 
   pub static ref BNE: Operation = Operation::new("BNE", "branch", vec![*IMPLIED,]);

 }
}

/// Branch on EQual
#[allow(non_snake_case)]
pub mod beq {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("BEQ", "branch", 0xF0, beq, 2, 1, 2, AddressingMode::Implied);
 
   pub static ref BEQ: Operation = Operation::new("BEQ", "branch", vec![*IMPLIED,]);

 }
}

/// BReaK
#[allow(non_snake_case)]
pub mod brk {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("BRK", "branch", 0x00, brk, 7, 0, 1, AddressingMode::Implied);
 
   pub static ref BRK: Operation = Operation::new("BRK", "branch", vec![*IMPLIED,]);

 }
}

/// CoMPare accumulator
#[allow(non_snake_case)]
pub mod cmp {
 use super::*;

 lazy_static! {
   pub static ref IMMEDIATE   : OpCode = OpCode::new("CMP", "logical", 0xC9, cmp, 2, 0, 2, AddressingMode::Immediate);
   pub static ref ZERO_PAGE   : OpCode = OpCode::new("CMP", "logical", 0xC5, cmp, 3, 0, 2, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X : OpCode = OpCode::new("CMP", "logical", 0xD5, cmp, 4, 0, 2, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE    : OpCode = OpCode::new("CMP", "logical", 0xCD, cmp, 4, 0, 3, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X  : OpCode = OpCode::new("CMP", "logical", 0xDD, cmp, 4, 1, 3, AddressingMode::AbsoluteX);
   pub static ref ABSOLUTE_Y  : OpCode = OpCode::new("CMP", "logical", 0xD9, cmp, 4, 1, 3, AddressingMode::AbsoluteY);
   pub static ref INDIRECT_X  : OpCode = OpCode::new("CMP", "logical", 0xC1, cmp, 6, 0, 2, AddressingMode::IndirectX);
   pub static ref INDIRECT_Y  : OpCode = OpCode::new("CMP", "logical", 0xD1, cmp, 5, 1, 2, AddressingMode::IndirectY);
 
   pub static ref CMP: Operation = Operation::new("CMP", "logical", vec![*IMMEDIATE,*ZERO_PAGE,*ZERO_PAGE_X,*ABSOLUTE,*ABSOLUTE_X,*ABSOLUTE_Y,*INDIRECT_X,*INDIRECT_Y,]);

 }
}

/// ComPare X register
#[allow(non_snake_case)]
pub mod cpx {
 use super::*;

 lazy_static! {
   pub static ref IMMEDIATE   : OpCode = OpCode::new("CPX", "logical", 0xE0, cpx, 2, 0, 2, AddressingMode::Immediate);
   pub static ref ZERO_PAGE   : OpCode = OpCode::new("CPX", "logical", 0xE4, cpx, 3, 0, 2, AddressingMode::ZeroPage);
   pub static ref ABSOLUTE    : OpCode = OpCode::new("CPX", "logical", 0xEC, cpx, 4, 0, 3, AddressingMode::Absolute);
 
   pub static ref CPX: Operation = Operation::new("CPX", "logical", vec![*IMMEDIATE,*ZERO_PAGE,*ABSOLUTE,]);

 }
}

/// ComPare Y register
#[allow(non_snake_case)]
pub mod cpy {
 use super::*;

 lazy_static! {
   pub static ref IMMEDIATE   : OpCode = OpCode::new("CPY", "logical", 0xC0, cpy, 2, 0, 2, AddressingMode::Immediate);
   pub static ref ZERO_PAGE   : OpCode = OpCode::new("CPY", "logical", 0xC4, cpy, 3, 0, 2, AddressingMode::ZeroPage);
   pub static ref ABSOLUTE    : OpCode = OpCode::new("CPY", "logical", 0xCC, cpy, 4, 0, 3, AddressingMode::Absolute);
 
   pub static ref CPY: Operation = Operation::new("CPY", "logical", vec![*IMMEDIATE,*ZERO_PAGE,*ABSOLUTE,]);

 }
}

/// DECrement memory
#[allow(non_snake_case)]
pub mod dec {
 use super::*;

 lazy_static! {
   pub static ref ZERO_PAGE   : OpCode = OpCode::new("DEC", "arithmatic", 0xC6, dec, 5, 0, 2, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X : OpCode = OpCode::new("DEC", "arithmatic", 0xD6, dec, 6, 0, 2, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE    : OpCode = OpCode::new("DEC", "arithmatic", 0xCE, dec, 6, 0, 3, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X  : OpCode = OpCode::new("DEC", "arithmatic", 0xDE, dec, 7, 0, 3, AddressingMode::AbsoluteX);
 
   pub static ref DEC: Operation = Operation::new("DEC", "arithmatic", vec![*ZERO_PAGE,*ZERO_PAGE_X,*ABSOLUTE,*ABSOLUTE_X,]);

 }
}

/// bitwise Exclusive OR
#[allow(non_snake_case)]
pub mod eor {
 use super::*;

 lazy_static! {
   pub static ref IMMEDIATE   : OpCode = OpCode::new("EOR", "arithmatic", 0x49, eor, 2, 0, 2, AddressingMode::Immediate);
   pub static ref ZERO_PAGE   : OpCode = OpCode::new("EOR", "arithmatic", 0x45, eor, 3, 0, 2, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X : OpCode = OpCode::new("EOR", "arithmatic", 0x55, eor, 4, 0, 2, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE    : OpCode = OpCode::new("EOR", "arithmatic", 0x4D, eor, 4, 0, 3, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X  : OpCode = OpCode::new("EOR", "arithmatic", 0x5D, eor, 4, 1, 3, AddressingMode::AbsoluteX);
   pub static ref ABSOLUTE_Y  : OpCode = OpCode::new("EOR", "arithmatic", 0x59, eor, 4, 1, 3, AddressingMode::AbsoluteY);
   pub static ref INDIRECT_X  : OpCode = OpCode::new("EOR", "arithmatic", 0x41, eor, 6, 0, 2, AddressingMode::IndirectX);
   pub static ref INDIRECT_Y  : OpCode = OpCode::new("EOR", "arithmatic", 0x51, eor, 5, 1, 2, AddressingMode::IndirectY);
 
   pub static ref EOR: Operation = Operation::new("EOR", "arithmatic", vec![*IMMEDIATE,*ZERO_PAGE,*ZERO_PAGE_X,*ABSOLUTE,*ABSOLUTE_X,*ABSOLUTE_Y,*INDIRECT_X,*INDIRECT_Y,]);

 }
}

/// CLear Carry
#[allow(non_snake_case)]
pub mod clc {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("CLC", "flag", 0x18, clc, 2, 0, 1, AddressingMode::Implied);
 
   pub static ref CLC: Operation = Operation::new("CLC", "flag", vec![*IMPLIED,]);

 }
}

/// SEt Carry
#[allow(non_snake_case)]
pub mod sec {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("SEC", "flag", 0x38, sec, 2, 0, 1, AddressingMode::Implied);
 
   pub static ref SEC: Operation = Operation::new("SEC", "flag", vec![*IMPLIED,]);

 }
}

/// CLear Interrupt
#[allow(non_snake_case)]
pub mod cli {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("CLI", "flag", 0x58, cli, 2, 0, 1, AddressingMode::Implied);
 
   pub static ref CLI: Operation = Operation::new("CLI", "flag", vec![*IMPLIED,]);

 }
}

/// SEt Interrupt
#[allow(non_snake_case)]
pub mod sei {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("SEI", "flag", 0x78, sei, 2, 0, 1, AddressingMode::Implied);
 
   pub static ref SEI: Operation = Operation::new("SEI", "flag", vec![*IMPLIED,]);

 }
}

/// CLear oVerflow
#[allow(non_snake_case)]
pub mod clv {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("CLV", "flag", 0xB8, clv, 2, 0, 1, AddressingMode::Implied);
 
   pub static ref CLV: Operation = Operation::new("CLV", "flag", vec![*IMPLIED,]);

 }
}

/// CLear Decimal
#[allow(non_snake_case)]
pub mod cld {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("CLD", "flag", 0xD8, cld, 2, 0, 1, AddressingMode::Implied);
 
   pub static ref CLD: Operation = Operation::new("CLD", "flag", vec![*IMPLIED,]);

 }
}

/// SEt Decimal
#[allow(non_snake_case)]
pub mod sed {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("SED", "flag", 0xF8, sed, 2, 0, 1, AddressingMode::Implied);
 
   pub static ref SED: Operation = Operation::new("SED", "flag", vec![*IMPLIED,]);

 }
}

/// INCrement memory
#[allow(non_snake_case)]
pub mod inc {
 use super::*;

 lazy_static! {
   pub static ref ZERO_PAGE   : OpCode = OpCode::new("INC", "arithmatic", 0xE6, inc, 5, 0, 2, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X : OpCode = OpCode::new("INC", "arithmatic", 0xF6, inc, 6, 0, 2, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE    : OpCode = OpCode::new("INC", "arithmatic", 0xEE, inc, 6, 0, 3, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X  : OpCode = OpCode::new("INC", "arithmatic", 0xFE, inc, 7, 0, 3, AddressingMode::AbsoluteX);
 
   pub static ref INC: Operation = Operation::new("INC", "arithmatic", vec![*ZERO_PAGE,*ZERO_PAGE_X,*ABSOLUTE,*ABSOLUTE_X,]);

 }
}

/// JuMP
#[allow(non_snake_case)]
pub mod jmp {
 use super::*;

 lazy_static! {
   pub static ref ABSOLUTE    : OpCode = OpCode::new("JMP", "branch", 0x4C, jmp, 3, 0, 3, AddressingMode::Absolute);
   pub static ref INDIRECT    : OpCode = OpCode::new("JMP", "branch", 0x6C, jmp, 5, 0, 3, AddressingMode::Indirect);
 
   pub static ref JMP: Operation = Operation::new("JMP", "branch", vec![*ABSOLUTE,*INDIRECT,]);

 }
}

/// Jump to SubRoutine
#[allow(non_snake_case)]
pub mod jsr {
 use super::*;

 lazy_static! {
   pub static ref ABSOLUTE    : OpCode = OpCode::new("JSR", "branch", 0x20, jsr, 6, 0, 3, AddressingMode::Absolute);
 
   pub static ref JSR: Operation = Operation::new("JSR", "branch", vec![*ABSOLUTE,]);

 }
}

/// LoaD Accumulator
#[allow(non_snake_case)]
pub mod lda {
 use super::*;

 lazy_static! {
   pub static ref IMMEDIATE   : OpCode = OpCode::new("LDA", "movement", 0xA9, lda, 2, 0, 2, AddressingMode::Immediate);
   pub static ref ZERO_PAGE   : OpCode = OpCode::new("LDA", "movement", 0xA5, lda, 3, 0, 2, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X : OpCode = OpCode::new("LDA", "movement", 0xB5, lda, 4, 0, 2, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE    : OpCode = OpCode::new("LDA", "movement", 0xAD, lda, 4, 0, 3, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X  : OpCode = OpCode::new("LDA", "movement", 0xBD, lda, 4, 1, 3, AddressingMode::AbsoluteX);
   pub static ref ABSOLUTE_Y  : OpCode = OpCode::new("LDA", "movement", 0xB9, lda, 4, 1, 3, AddressingMode::AbsoluteY);
   pub static ref INDIRECT_X  : OpCode = OpCode::new("LDA", "movement", 0xA1, lda, 6, 0, 2, AddressingMode::IndirectX);
   pub static ref INDIRECT_Y  : OpCode = OpCode::new("LDA", "movement", 0xB1, lda, 5, 1, 2, AddressingMode::IndirectY);
 
   pub static ref LDA: Operation = Operation::new("LDA", "movement", vec![*IMMEDIATE,*ZERO_PAGE,*ZERO_PAGE_X,*ABSOLUTE,*ABSOLUTE_X,*ABSOLUTE_Y,*INDIRECT_X,*INDIRECT_Y,]);

 }
}

/// LoaD X register
#[allow(non_snake_case)]
pub mod ldx {
 use super::*;

 lazy_static! {
   pub static ref IMMEDIATE   : OpCode = OpCode::new("LDX", "movement", 0xA2, ldx, 2, 0, 2, AddressingMode::Immediate);
   pub static ref ZERO_PAGE   : OpCode = OpCode::new("LDX", "movement", 0xA6, ldx, 3, 0, 2, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_Y : OpCode = OpCode::new("LDX", "movement", 0xB6, ldx, 4, 0, 2, AddressingMode::ZeroPageY);
   pub static ref ABSOLUTE    : OpCode = OpCode::new("LDX", "movement", 0xAE, ldx, 4, 0, 3, AddressingMode::Absolute);
   pub static ref ABSOLUTE_Y  : OpCode = OpCode::new("LDX", "movement", 0xBE, ldx, 4, 1, 3, AddressingMode::AbsoluteY);
 
   pub static ref LDX: Operation = Operation::new("LDX", "movement", vec![*IMMEDIATE,*ZERO_PAGE,*ZERO_PAGE_Y,*ABSOLUTE,*ABSOLUTE_Y,]);

 }
}

/// LoaD Y register
#[allow(non_snake_case)]
pub mod ldy {
 use super::*;

 lazy_static! {
   pub static ref IMMEDIATE   : OpCode = OpCode::new("LDY", "movement", 0xA0, ldy, 2, 0, 2, AddressingMode::Immediate);
   pub static ref ZERO_PAGE   : OpCode = OpCode::new("LDY", "movement", 0xA4, ldy, 3, 0, 2, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X : OpCode = OpCode::new("LDY", "movement", 0xB4, ldy, 4, 0, 2, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE    : OpCode = OpCode::new("LDY", "movement", 0xAC, ldy, 4, 0, 3, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X  : OpCode = OpCode::new("LDY", "movement", 0xBC, ldy, 4, 1, 3, AddressingMode::AbsoluteX);
 
   pub static ref LDY: Operation = Operation::new("LDY", "movement", vec![*IMMEDIATE,*ZERO_PAGE,*ZERO_PAGE_X,*ABSOLUTE,*ABSOLUTE_X,]);

 }
}

/// Logical Shift Right
#[allow(non_snake_case)]
pub mod lsr {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("LSR", "arithmatic", 0x4A, lsr, 2, 0, 1, AddressingMode::Implied);
   pub static ref ZERO_PAGE   : OpCode = OpCode::new("LSR", "arithmatic", 0x46, lsr, 5, 0, 2, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X : OpCode = OpCode::new("LSR", "arithmatic", 0x56, lsr, 6, 0, 2, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE    : OpCode = OpCode::new("LSR", "arithmatic", 0x4E, lsr, 6, 0, 3, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X  : OpCode = OpCode::new("LSR", "arithmatic", 0x5E, lsr, 7, 0, 3, AddressingMode::AbsoluteX);
 
   pub static ref LSR: Operation = Operation::new("LSR", "arithmatic", vec![*IMPLIED,*ZERO_PAGE,*ZERO_PAGE_X,*ABSOLUTE,*ABSOLUTE_X,]);

 }
}

/// No OPeration
#[allow(non_snake_case)]
pub mod nop {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("NOP", "movement", 0xEA, nop, 2, 0, 1, AddressingMode::Implied);
 
   pub static ref NOP: Operation = Operation::new("NOP", "movement", vec![*IMPLIED,]);

 }
}

/// bitwise OR with Accumulator
#[allow(non_snake_case)]
pub mod ora {
 use super::*;

 lazy_static! {
   pub static ref IMMEDIATE   : OpCode = OpCode::new("ORA", "arithmatic", 0x09, ora, 2, 0, 2, AddressingMode::Immediate);
   pub static ref ZERO_PAGE   : OpCode = OpCode::new("ORA", "arithmatic", 0x05, ora, 3, 0, 2, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X : OpCode = OpCode::new("ORA", "arithmatic", 0x15, ora, 4, 0, 2, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE    : OpCode = OpCode::new("ORA", "arithmatic", 0x0D, ora, 4, 0, 3, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X  : OpCode = OpCode::new("ORA", "arithmatic", 0x1D, ora, 4, 1, 3, AddressingMode::AbsoluteX);
   pub static ref ABSOLUTE_Y  : OpCode = OpCode::new("ORA", "arithmatic", 0x19, ora, 4, 1, 3, AddressingMode::AbsoluteY);
   pub static ref INDIRECT_X  : OpCode = OpCode::new("ORA", "arithmatic", 0x01, ora, 6, 0, 2, AddressingMode::IndirectX);
   pub static ref INDIRECT_Y  : OpCode = OpCode::new("ORA", "arithmatic", 0x11, ora, 5, 1, 2, AddressingMode::IndirectY);
 
   pub static ref ORA: Operation = Operation::new("ORA", "arithmatic", vec![*IMMEDIATE,*ZERO_PAGE,*ZERO_PAGE_X,*ABSOLUTE,*ABSOLUTE_X,*ABSOLUTE_Y,*INDIRECT_X,*INDIRECT_Y,]);

 }
}

/// Transfer Accumulator to X
#[allow(non_snake_case)]
pub mod tax {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("TAX", "movement", 0xAA, tax, 2, 0, 1, AddressingMode::Implied);
 
   pub static ref TAX: Operation = Operation::new("TAX", "movement", vec![*IMPLIED,]);

 }
}

/// Transfer X to Accumulator
#[allow(non_snake_case)]
pub mod txa {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("TXA", "movement", 0x8A, txa, 2, 0, 1, AddressingMode::Implied);
 
   pub static ref TXA: Operation = Operation::new("TXA", "movement", vec![*IMPLIED,]);

 }
}

/// DEcrement X
#[allow(non_snake_case)]
pub mod dex {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("DEX", "movement", 0xCA, dex, 2, 0, 1, AddressingMode::Implied);
 
   pub static ref DEX: Operation = Operation::new("DEX", "movement", vec![*IMPLIED,]);

 }
}

/// INcrement X
#[allow(non_snake_case)]
pub mod inx {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("INX", "movement", 0xE8, inx, 2, 0, 1, AddressingMode::Implied);
 
   pub static ref INX: Operation = Operation::new("INX", "movement", vec![*IMPLIED,]);

 }
}

/// Transfer Accumulator to Y
#[allow(non_snake_case)]
pub mod tay {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("TAY", "movement", 0xA8, tay, 2, 0, 1, AddressingMode::Implied);
 
   pub static ref TAY: Operation = Operation::new("TAY", "movement", vec![*IMPLIED,]);

 }
}

/// Transfer Y to Accumulator
#[allow(non_snake_case)]
pub mod tya {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("TYA", "movement", 0x98, tya, 2, 0, 1, AddressingMode::Implied);
 
   pub static ref TYA: Operation = Operation::new("TYA", "movement", vec![*IMPLIED,]);

 }
}

/// DEcrement Y
#[allow(non_snake_case)]
pub mod dey {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("DEY", "movement", 0x88, dey, 2, 0, 1, AddressingMode::Implied);
 
   pub static ref DEY: Operation = Operation::new("DEY", "movement", vec![*IMPLIED,]);

 }
}

/// INcrement Y
#[allow(non_snake_case)]
pub mod iny {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("INY", "movement", 0xC8, iny, 2, 0, 1, AddressingMode::Implied);
 
   pub static ref INY: Operation = Operation::new("INY", "movement", vec![*IMPLIED,]);

 }
}

/// ROtate Left
#[allow(non_snake_case)]
pub mod rol {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("ROL", "arithmatic", 0x2A, rol, 2, 0, 1, AddressingMode::Implied);
   pub static ref ZERO_PAGE   : OpCode = OpCode::new("ROL", "arithmatic", 0x26, rol, 5, 0, 2, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X : OpCode = OpCode::new("ROL", "arithmatic", 0x36, rol, 6, 0, 2, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE    : OpCode = OpCode::new("ROL", "arithmatic", 0x2E, rol, 6, 0, 3, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X  : OpCode = OpCode::new("ROL", "arithmatic", 0x3E, rol, 7, 0, 3, AddressingMode::AbsoluteX);
 
   pub static ref ROL: Operation = Operation::new("ROL", "arithmatic", vec![*IMPLIED,*ZERO_PAGE,*ZERO_PAGE_X,*ABSOLUTE,*ABSOLUTE_X,]);

 }
}

/// ROtate Right
#[allow(non_snake_case)]
pub mod ror {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("ROR", "arithmatic", 0x6A, ror, 2, 0, 1, AddressingMode::Implied);
   pub static ref ZERO_PAGE   : OpCode = OpCode::new("ROR", "arithmatic", 0x66, ror, 5, 0, 2, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X : OpCode = OpCode::new("ROR", "arithmatic", 0x76, ror, 6, 0, 2, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE    : OpCode = OpCode::new("ROR", "arithmatic", 0x6E, ror, 6, 0, 3, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X  : OpCode = OpCode::new("ROR", "arithmatic", 0x7E, ror, 7, 0, 3, AddressingMode::AbsoluteX);
 
   pub static ref ROR: Operation = Operation::new("ROR", "arithmatic", vec![*IMPLIED,*ZERO_PAGE,*ZERO_PAGE_X,*ABSOLUTE,*ABSOLUTE_X,]);

 }
}

/// ReTurn from Interrupt
#[allow(non_snake_case)]
pub mod rti {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("RTI", "branch", 0x40, rti, 6, 0, 1, AddressingMode::Implied);
 
   pub static ref RTI: Operation = Operation::new("RTI", "branch", vec![*IMPLIED,]);

 }
}

/// ReTurn from Subroutine
#[allow(non_snake_case)]
pub mod rts {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("RTS", "branch", 0x60, rts, 6, 0, 1, AddressingMode::Implied);
 
   pub static ref RTS: Operation = Operation::new("RTS", "branch", vec![*IMPLIED,]);

 }
}

/// SuBtract with Carry
#[allow(non_snake_case)]
pub mod sbc {
 use super::*;

 lazy_static! {
   pub static ref IMMEDIATE   : OpCode = OpCode::new("SBC", "arithmatic", 0xE9, sbc, 2, 0, 2, AddressingMode::Immediate);
   pub static ref ZERO_PAGE   : OpCode = OpCode::new("SBC", "arithmatic", 0xE5, sbc, 3, 0, 2, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X : OpCode = OpCode::new("SBC", "arithmatic", 0xF5, sbc, 4, 0, 2, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE    : OpCode = OpCode::new("SBC", "arithmatic", 0xED, sbc, 4, 0, 3, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X  : OpCode = OpCode::new("SBC", "arithmatic", 0xFD, sbc, 4, 1, 3, AddressingMode::AbsoluteX);
   pub static ref ABSOLUTE_Y  : OpCode = OpCode::new("SBC", "arithmatic", 0xF9, sbc, 4, 1, 3, AddressingMode::AbsoluteY);
   pub static ref INDIRECT_X  : OpCode = OpCode::new("SBC", "arithmatic", 0xE1, sbc, 6, 0, 2, AddressingMode::IndirectX);
   pub static ref INDIRECT_Y  : OpCode = OpCode::new("SBC", "arithmatic", 0xF1, sbc, 5, 1, 2, AddressingMode::IndirectY);
 
   pub static ref SBC: Operation = Operation::new("SBC", "arithmatic", vec![*IMMEDIATE,*ZERO_PAGE,*ZERO_PAGE_X,*ABSOLUTE,*ABSOLUTE_X,*ABSOLUTE_Y,*INDIRECT_X,*INDIRECT_Y,]);

 }
}

/// STore Accumulator
#[allow(non_snake_case)]
pub mod sta {
 use super::*;

 lazy_static! {
   pub static ref ZERO_PAGE   : OpCode = OpCode::new("STA", "movement", 0x85, sta, 3, 0, 2, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X : OpCode = OpCode::new("STA", "movement", 0x95, sta, 4, 0, 2, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE    : OpCode = OpCode::new("STA", "movement", 0x8D, sta, 4, 0, 3, AddressingMode::Absolute);
   pub static ref ABSOLUTE_X  : OpCode = OpCode::new("STA", "movement", 0x9D, sta, 5, 0, 3, AddressingMode::AbsoluteX);
   pub static ref ABSOLUTE_Y  : OpCode = OpCode::new("STA", "movement", 0x99, sta, 5, 0, 3, AddressingMode::AbsoluteY);
   pub static ref INDIRECT_X  : OpCode = OpCode::new("STA", "movement", 0x81, sta, 6, 0, 2, AddressingMode::IndirectX);
   pub static ref INDIRECT_Y  : OpCode = OpCode::new("STA", "movement", 0x91, sta, 6, 0, 2, AddressingMode::IndirectY);
 
   pub static ref STA: Operation = Operation::new("STA", "movement", vec![*ZERO_PAGE,*ZERO_PAGE_X,*ABSOLUTE,*ABSOLUTE_X,*ABSOLUTE_Y,*INDIRECT_X,*INDIRECT_Y,]);

 }
}

/// Transfer X to Stack ptr
#[allow(non_snake_case)]
pub mod txs {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("TXS", "movement", 0x9A, txs, 2, 0, 1, AddressingMode::Implied);
 
   pub static ref TXS: Operation = Operation::new("TXS", "movement", vec![*IMPLIED,]);

 }
}

/// Transfer Stack ptr to X
#[allow(non_snake_case)]
pub mod tsx {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("TSX", "movement", 0xBA, tsx, 2, 0, 1, AddressingMode::Implied);
 
   pub static ref TSX: Operation = Operation::new("TSX", "movement", vec![*IMPLIED,]);

 }
}

/// PusH Accumulator
#[allow(non_snake_case)]
pub mod pha {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("PHA", "stack", 0x48, pha, 3, 0, 1, AddressingMode::Implied);
 
   pub static ref PHA: Operation = Operation::new("PHA", "stack", vec![*IMPLIED,]);

 }
}

/// PuLl Accumulator
#[allow(non_snake_case)]
pub mod pla {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("PLA", "stack", 0x68, pla, 4, 0, 1, AddressingMode::Implied);
 
   pub static ref PLA: Operation = Operation::new("PLA", "stack", vec![*IMPLIED,]);

 }
}

/// PusH Processor status
#[allow(non_snake_case)]
pub mod php {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("PHP", "stack", 0x08, php, 3, 0, 1, AddressingMode::Implied);
 
   pub static ref PHP: Operation = Operation::new("PHP", "stack", vec![*IMPLIED,]);

 }
}

/// PuLl Processor status
#[allow(non_snake_case)]
pub mod plp {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED     : OpCode = OpCode::new("PLP", "stack", 0x28, plp, 4, 0, 1, AddressingMode::Implied);
 
   pub static ref PLP: Operation = Operation::new("PLP", "stack", vec![*IMPLIED,]);

 }
}

/// STore X register
#[allow(non_snake_case)]
pub mod stx {
 use super::*;

 lazy_static! {
   pub static ref ZERO_PAGE   : OpCode = OpCode::new("STX", "movement", 0x86, stx, 3, 0, 2, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_Y : OpCode = OpCode::new("STX", "movement", 0x96, stx, 4, 0, 2, AddressingMode::ZeroPageY);
   pub static ref ABSOLUTE    : OpCode = OpCode::new("STX", "movement", 0x8E, stx, 4, 0, 3, AddressingMode::Absolute);
 
   pub static ref STX: Operation = Operation::new("STX", "movement", vec![*ZERO_PAGE,*ZERO_PAGE_Y,*ABSOLUTE,]);

 }
}

/// STore Y register
#[allow(non_snake_case)]
pub mod sty {
 use super::*;

 lazy_static! {
   pub static ref ZERO_PAGE   : OpCode = OpCode::new("STY", "movement", 0x84, sty, 3, 0, 2, AddressingMode::ZeroPage);
   pub static ref ZERO_PAGE_X : OpCode = OpCode::new("STY", "movement", 0x94, sty, 4, 0, 2, AddressingMode::ZeroPageX);
   pub static ref ABSOLUTE    : OpCode = OpCode::new("STY", "movement", 0x8C, sty, 4, 0, 3, AddressingMode::Absolute);
 
   pub static ref STY: Operation = Operation::new("STY", "movement", vec![*ZERO_PAGE,*ZERO_PAGE_X,*ABSOLUTE,]);

 }
}

/// Undocumented No-Op
#[allow(non_snake_case)]
pub mod undoc_nop {
 use super::*;

 lazy_static! {
   pub static ref IMPLIED: OpCode = OpCode::new("undoc_nop", "no-op", 0xEA, undoc_nop, 2, 0, 1, AddressingMode::Implied);
 
   pub static ref UNDOC_NOP: Operation = Operation::new("undoc_nop", "no-op", vec![*IMPLIED,]);

 }
}



 lazy_static! {

 
/// This is the optable, which contains all of the opcodes for the 6502 CPU.
/// The order of the ops is **EXTREMELY** important, as the index is the opcode byte.
/// Any opcode marked as undoc_nop is an undocumented opcode, and will be logged when executed, but will not do anything.
/// The optable is like a huge match statement, but is **SIGNIFICANTLY** faster because it is a static array.
 
    pub static ref OPTABLE: [OpCode; 256] = [
        *brk::IMPLIED,
        *ora::INDIRECT_X,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *ora::ZERO_PAGE,
        *asl::ZERO_PAGE,
        *undoc_nop::IMPLIED,
        *php::IMPLIED,
        *ora::IMMEDIATE,
        *asl::IMPLIED,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *ora::ABSOLUTE,
        *asl::ABSOLUTE,
        *undoc_nop::IMPLIED,
        *bpl::IMPLIED,
        *ora::INDIRECT_Y,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *ora::ZERO_PAGE_X,
        *asl::ZERO_PAGE_X,
        *undoc_nop::IMPLIED,
        *clc::IMPLIED,
        *ora::ABSOLUTE_Y,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *ora::ABSOLUTE_X,
        *asl::ABSOLUTE_X,
        *undoc_nop::IMPLIED,
        *jsr::ABSOLUTE,
        *and::INDIRECT_X,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *bit::ZERO_PAGE,
        *and::ZERO_PAGE,
        *rol::ZERO_PAGE,
        *undoc_nop::IMPLIED,
        *plp::IMPLIED,
        *and::IMMEDIATE,
        *rol::IMPLIED,
        *undoc_nop::IMPLIED,
        *bit::ABSOLUTE,
        *and::ABSOLUTE,
        *rol::ABSOLUTE,
        *undoc_nop::IMPLIED,
        *bmi::IMPLIED,
        *and::INDIRECT_Y,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *and::ZERO_PAGE_X,
        *rol::ZERO_PAGE_X,
        *undoc_nop::IMPLIED,
        *sec::IMPLIED,
        *and::ABSOLUTE_Y,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *and::ABSOLUTE_X,
        *rol::ABSOLUTE_X,
        *undoc_nop::IMPLIED,
        *rti::IMPLIED,
        *eor::INDIRECT_X,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *eor::ZERO_PAGE,
        *lsr::ZERO_PAGE,
        *undoc_nop::IMPLIED,
        *pha::IMPLIED,
        *eor::IMMEDIATE,
        *lsr::IMPLIED,
        *undoc_nop::IMPLIED,
        *jmp::ABSOLUTE,
        *eor::ABSOLUTE,
        *lsr::ABSOLUTE,
        *undoc_nop::IMPLIED,
        *bvc::IMPLIED,
        *eor::INDIRECT_Y,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *eor::ZERO_PAGE_X,
        *lsr::ZERO_PAGE_X,
        *undoc_nop::IMPLIED,
        *cli::IMPLIED,
        *eor::ABSOLUTE_Y,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *eor::ABSOLUTE_X,
        *lsr::ABSOLUTE_X,
        *undoc_nop::IMPLIED,
        *rts::IMPLIED,
        *adc::INDIRECT_X,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *adc::ZERO_PAGE,
        *ror::ZERO_PAGE,
        *undoc_nop::IMPLIED,
        *pla::IMPLIED,
        *adc::IMMEDIATE,
        *ror::IMPLIED,
        *undoc_nop::IMPLIED,
        *jmp::INDIRECT,
        *adc::ABSOLUTE,
        *ror::ABSOLUTE,
        *undoc_nop::IMPLIED,
        *bvs::IMPLIED,
        *adc::INDIRECT_Y,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *adc::ZERO_PAGE_X,
        *ror::ZERO_PAGE_X,
        *undoc_nop::IMPLIED,
        *sei::IMPLIED,
        *adc::ABSOLUTE_Y,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *adc::ABSOLUTE_X,
        *ror::ABSOLUTE_X,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *sta::INDIRECT_X,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *sty::ZERO_PAGE,
        *sta::ZERO_PAGE,
        *stx::ZERO_PAGE,
        *undoc_nop::IMPLIED,
        *dey::IMPLIED,
        *undoc_nop::IMPLIED,
        *txa::IMPLIED,
        *undoc_nop::IMPLIED,
        *sty::ABSOLUTE,
        *sta::ABSOLUTE,
        *stx::ABSOLUTE,
        *undoc_nop::IMPLIED,
        *bcc::IMPLIED,
        *sta::INDIRECT_Y,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *sty::ZERO_PAGE_X,
        *sta::ZERO_PAGE_X,
        *stx::ZERO_PAGE_Y,
        *undoc_nop::IMPLIED,
        *tya::IMPLIED,
        *sta::ABSOLUTE_Y,
        *txs::IMPLIED,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *sta::ABSOLUTE_X,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *ldy::IMMEDIATE,
        *lda::INDIRECT_X,
        *ldx::IMMEDIATE,
        *undoc_nop::IMPLIED,
        *ldy::ZERO_PAGE,
        *lda::ZERO_PAGE,
        *ldx::ZERO_PAGE,
        *undoc_nop::IMPLIED,
        *tay::IMPLIED,
        *lda::IMMEDIATE,
        *tax::IMPLIED,
        *undoc_nop::IMPLIED,
        *ldy::ABSOLUTE,
        *lda::ABSOLUTE,
        *ldx::ABSOLUTE,
        *undoc_nop::IMPLIED,
        *bcs::IMPLIED,
        *lda::INDIRECT_Y,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *ldy::ZERO_PAGE_X,
        *lda::ZERO_PAGE_X,
        *ldx::ZERO_PAGE_Y,
        *undoc_nop::IMPLIED,
        *clv::IMPLIED,
        *lda::ABSOLUTE_Y,
        *tsx::IMPLIED,
        *undoc_nop::IMPLIED,
        *ldy::ABSOLUTE_X,
        *lda::ABSOLUTE_X,
        *ldx::ABSOLUTE_Y,
        *undoc_nop::IMPLIED,
        *cpy::IMMEDIATE,
        *cmp::INDIRECT_X,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *cpy::ZERO_PAGE,
        *cmp::ZERO_PAGE,
        *dec::ZERO_PAGE,
        *undoc_nop::IMPLIED,
        *iny::IMPLIED,
        *cmp::IMMEDIATE,
        *dex::IMPLIED,
        *undoc_nop::IMPLIED,
        *cpy::ABSOLUTE,
        *cmp::ABSOLUTE,
        *dec::ABSOLUTE,
        *undoc_nop::IMPLIED,
        *bne::IMPLIED,
        *cmp::INDIRECT_Y,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *cmp::ZERO_PAGE_X,
        *dec::ZERO_PAGE_X,
        *undoc_nop::IMPLIED,
        *cld::IMPLIED,
        *cmp::ABSOLUTE_Y,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *cmp::ABSOLUTE_X,
        *dec::ABSOLUTE_X,
        *undoc_nop::IMPLIED,
        *cpx::IMMEDIATE,
        *sbc::INDIRECT_X,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *cpx::ZERO_PAGE,
        *sbc::ZERO_PAGE,
        *inc::ZERO_PAGE,
        *undoc_nop::IMPLIED,
        *inx::IMPLIED,
        *sbc::IMMEDIATE,
        *nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *cpx::ABSOLUTE,
        *sbc::ABSOLUTE,
        *inc::ABSOLUTE,
        *undoc_nop::IMPLIED,
        *beq::IMPLIED,
        *sbc::INDIRECT_Y,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *sbc::ZERO_PAGE_X,
        *inc::ZERO_PAGE_X,
        *undoc_nop::IMPLIED,
        *sed::IMPLIED,
        *sbc::ABSOLUTE_Y,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *undoc_nop::IMPLIED,
        *sbc::ABSOLUTE_X,
        *inc::ABSOLUTE_X,
        *undoc_nop::IMPLIED,
    ];
}
