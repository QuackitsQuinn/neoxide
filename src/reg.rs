///
/// Register trait
/// 
/// This trait is used to define the interface for registers. 
pub trait Register<T> {
    /// Read the value of the register
    fn read(&self) -> T;
    /// Write a value to the register
    fn write(&mut self, value: T);
}

pub struct Reg {
    value: u8,
}

impl Reg {
    pub fn new(value: u8) -> Self {
        Reg { value }
    }
    fn incr(&self) {
        self.value+=1;
    }
    fn decr(&self) {
        self.value-=1;
    }
}

impl Register<u8> for Reg {
    fn read(&self) -> u8 {
        self.value
    }

    fn write(&mut self, value: u8) {
        self.value = value;
    }
}