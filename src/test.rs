use std::ops::Add;

use crate::{cpu::CPU, ops::opcode::OpCode};


/// Creates a CPU with the given ops loaded into memory at 0x0000
/// Offset is set to 0x0000 to make 6502 assembly easier to write and not have to offset everything by 0x0600
pub fn config_cpu(ops: Vec<u8>) -> CPU {
    let mut cpu = CPU::new();
    cpu.set_load_offset(0);
    cpu.load_vec(ops);
    cpu
}
/// Helpful struct for building 6502 programs
pub struct ProgramBuilder(Vec<u8>);

impl Default for ProgramBuilder {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl ProgramBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn op(&mut self, op: OpCode) -> &mut Self {
        self.0.push(op.into());
        self
    }

    pub fn ops(&mut self, ops: &[u8]) -> &mut Self {
        self.0.extend_from_slice(ops);
        self
    }

    pub fn addr(&mut self, addr: u16) -> &mut Self {
        self.0.extend_from_slice(&addr.to_le_bytes());
        self
    }

    pub fn byte(&mut self, byte: u8) -> &mut Self {
        self.0.push(byte);
        self
    }

    pub fn bytes(&self)-> Vec<u8> {
        self.0.clone()
    }
}