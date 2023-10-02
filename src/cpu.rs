use crate::reg::{U8Register, ProgramCounter, Register};


pub struct CPU {
    a: U8Register,
    x: U8Register,
    y: U8Register,
    pub pc: ProgramCounter
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            a: U8Register::new(0),
            x: U8Register::new(0),
            y: U8Register::new(0),
            pc: ProgramCounter::new(0)
        }
    }

    pub fn reset(&mut self) {
        self.a.write(0);
        self.x.write(0);
        self.y.write(0);
        self.pc.reset();
    }
}
