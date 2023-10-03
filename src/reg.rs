use std::ops;

///
/// Register trait
/// 
/// This trait is used to define the interface for registers. 
pub trait Register<T>: ops::AddAssign<T> + ops::SubAssign<T> {
    /// Read the value of the register
    fn read(&self) -> T;
    /// Write a value to the register
    fn write(&mut self, value: T);
    /// Increment the value of the register
    fn incr(&mut self);
    /// Decrement the value of the register
    fn decr(&mut self);
}
/// A simple u8 register
pub struct U8Register {
    value: u8,
}

impl U8Register {
    pub fn new(value: u8) -> Self {
        U8Register { value }
    }
}

impl ops::AddAssign<u8> for U8Register {
    fn add_assign(&mut self, rhs: u8) {
        self.value += rhs;
    }
}

impl ops::SubAssign<u8> for U8Register {
    fn sub_assign(&mut self, rhs: u8) {
        self.value -= rhs;
    }
}

impl Register<u8> for U8Register {
    fn read(&self) -> u8 {
        self.value
    }

    fn write(&mut self, value: u8) {
        self.value = value;
    }
    fn incr(&mut self) {
        self.value+=1;
    }
    fn decr(&mut self) {
        self.value-=1;
    }
}
impl Into<u8> for U8Register {
    fn into(self) -> u8 {
        self.value
    }
}
/// U16 register intended to be used as a program counter
pub struct ProgramCounter {
    pc: u16,
    pgrm_start: u16,
}

impl ProgramCounter {
    /// Create a new program counter
    pub fn new(pc: u16) -> Self {
        ProgramCounter { pc, pgrm_start: pc }
    }
    /// Reset the program counter to the start of the program
    pub fn reset(&mut self) {
        self.pc = self.pgrm_start;
    }
    /// Sets the entry point of the program
    pub fn set_entry_point(&mut self, entry_point: u16) {
        self.pgrm_start = entry_point;
    }
}
impl ops::AddAssign<u16> for ProgramCounter {
    fn add_assign(&mut self, rhs: u16) {
        self.pc += rhs;
    }
}

impl ops::SubAssign<u16> for ProgramCounter {
    fn sub_assign(&mut self, rhs: u16) {
        self.pc -= rhs;
    }
}
impl Register<u16> for ProgramCounter {
    fn read(&self) -> u16 {
        self.pc
    }

    fn write(&mut self, value: u16) {
        self.pc = value;
    }

    fn incr(&mut self) {
        self.pc += 1;
    }

    fn decr(&mut self) {
        self.pc -= 1;
    }
}

impl Into<u16> for ProgramCounter {
    fn into(self) -> u16 {
        self.pc
    }
}

#[cfg(test)]
mod test_pgrm_counter {
    use super::*;

    fn new(start: u16) -> ProgramCounter {
        ProgramCounter::new(0)
    }
}
