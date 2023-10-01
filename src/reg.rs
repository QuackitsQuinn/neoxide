///
/// Register trait
/// 
/// This trait is used to define the interface for registers. 
pub trait Register<T>{
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
pub struct Reg {
    value: u8,
}

impl Reg {
    pub fn new(value: u8) -> Self {
        Reg { value }
    }
}

impl Register<u8> for Reg {
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
/// U16 register intended to be used as a program counter
pub struct ProgramCounter {
    pc: u16,
    pgrm_start: u16,
}

impl ProgramCounter {
    pub fn new(pc: u16) -> Self {
        ProgramCounter { pc, pgrm_start: pc }
    }
    
    pub fn reset(&mut self) {
        self.pc = self.pgrm_start;
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

