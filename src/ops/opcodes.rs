use crate::{addressing::AddressingMode, cpu::CPU, ops::opcode::Operation};

fn tst(cpu: &mut CPU, mode: AddressingMode) {
}
pub mod TST {
use crate::{ops::opcode::Operation, addressing::AddressingMode};

use super::*;
    lazy_static! {
        pub static ref Implied: Operation = Operation::new("TST", 0, tst, 0, 0,AddressingMode::Implied);
    }
}

lazy_static! {
    pub static ref OPCODES: [Operation; 1] = [
        *TST::Implied,
    ];
}