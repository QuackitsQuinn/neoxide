use crate::{addressing::AddressingMode, cpu::CPU};

#[derive(Clone, Copy)]
pub struct OpCode {
    pub name: &'static str,
    pub optype: &'static str,
    pub code: u8,
    pub op: fn(&mut CPU, AddressingMode),
    pub cycles: u8,
    // This is the number of cycles to add if the operation crosses a page boundary  (0 if it doesn't)
    pub page_cross_incr: u8,
    pub length: u8, // including opcode
    pub mode: AddressingMode,
}

impl OpCode {
    pub fn new(
        name: &'static str,
        optype: &'static str,
        code: u8,
        op: fn(&mut CPU, AddressingMode),
        cycles: u8,
        page_cross_incr: u8,
        length: u8,
        mode: AddressingMode,
    ) -> Self {
        Self {
            name,
            optype,
            code,
            op,
            cycles,
            length,
            page_cross_incr,
            mode,
        }
    }
}
impl From<OpCode> for u8 {
    fn from(op: OpCode) -> Self {
        op.code
    }
}
/// Contains all the opcodes for a certain operation
pub struct Operation {
    pub name: &'static str,
    pub type_name: &'static str,
    pub ops: Vec<OpCode>,
    pub codes: Vec<u8>,
}

impl Operation {
    pub fn new(name: &'static str, type_name: &'static str, ops: Vec<OpCode>) -> Self {
        let mut codes = Vec::new();
        for op in &ops {
            codes.push(op.code);
        }
        Self {
            name,
            type_name,
            ops,
            codes,
        }
    }
}