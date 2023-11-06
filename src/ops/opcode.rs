use crate::{addressing::AddressingMode, cpu::CPU};

#[derive(Clone, Copy)]
pub struct Operation {
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

impl Operation {
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
// ideally this will be generated from a build script (do build scripts )
