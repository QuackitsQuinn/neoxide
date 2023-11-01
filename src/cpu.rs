use crate::{
    addressing::AddressingMode,
    cpu_flags::CPUStatus,
    memory::Memory,
    reg::{ProgramCounter, Register, U8Register},
    stack::Stack, constant::PGRM_LOAD_OFFSET,
};
/// A struct representing the CPU of the NES
#[derive(Debug)]
pub struct CPU {
    pub a: U8Register,
    pub x: U8Register,
    pub y: U8Register,
    pub pc: ProgramCounter,
    pub mem: Memory,
    pub status: CPUStatus,
    pub stack: Stack,
}

impl CPU {
    /// Create a new CPU
    pub fn new() -> Self {
        CPU {
            a: U8Register::new(0),
            x: U8Register::new(0),
            y: U8Register::new(0),
            pc: ProgramCounter::new(0),
            mem: Memory::new(),
            status: CPUStatus::new(),
            stack: Stack::new(),
        }
    }
    /// Reset the CPU to its initial state
    pub fn reset(&mut self) {
        self.a.write(0);
        self.x.write(0);
        self.y.write(0);
        self.pc.reset();
        self.mem.reset()
    }
    /// Read the next byte from the program counter
    pub fn read_opbyte(&mut self) -> u8 {
        self.pc.incr();
        self.mem.read_u8(self.pc.read()) // this is a bit weird but it works
    }
    /// Reads a u16 from the program counter
    pub fn read_u16(&mut self) -> u16 {
        self.pc.incr();
        let low = self.mem.read_u8(self.pc.read()) as u16;
        self.pc.incr();
        let high = self.mem.read_u8(self.pc.read()) as u16;
        (high << 8) | low
    }

    pub fn read_i16(&mut self) -> i16 {
        self.pc.incr();
        let low = self.mem.read_u8(self.pc.read()) as i16;
        self.pc.incr();
        let high = self.mem.read_u8(self.pc.read()) as i16;
        (high << 8) | low
    }
    /// Read the value at the address specified by the parameter
    pub fn read(&mut self, addr: u16) -> u8 {
        self.mem.read_u8(addr)
    }
    /// Write the value to the address specified by the parameter
    pub fn write(&mut self, addr: u16, data: u8) {
        self.mem.write(addr, data);
    }
    /// Read the value at the address specified by the addressing mode and the program counter
    pub fn get_addr(&mut self, admod: AddressingMode) -> u16 {
        match admod {
            AddressingMode::Immediate => {
                panic!("Immediate addressing mode does not have an address")
            } //
            AddressingMode::ZeroPage => self.read_opbyte() as u16,
            AddressingMode::ZeroPageX => {
                let opbyte = self.read_opbyte();
                (opbyte.wrapping_add(self.x.read())) as u16
            }
            AddressingMode::ZeroPageY => {
                let opbyte = self.read_opbyte();
                (opbyte.wrapping_add(self.y.read())) as u16
            }
            AddressingMode::Absolute => self.read_u16(),
            AddressingMode::AbsoluteX => {
                let addr = self.read_u16();
                addr.wrapping_add(self.x.read() as u16)
            }
            AddressingMode::AbsoluteY => {
                let addr = self.read_u16();
                addr.wrapping_add(self.y.read() as u16)
            }
            AddressingMode::IndirectX => {
                // i like ptrptr
                let ptrptr = self.read_opbyte().wrapping_add(self.x.read());
                let ptr = self.read(ptrptr as u16) as u16;
                let low = self.read(ptr) as u16;
                let high = self.read(ptr.wrapping_add(1)) as u16;
                (high << 8) | low
            }
            AddressingMode::IndirectY => {
                let ptrptr = self.read_opbyte().wrapping_add(self.y.read());
                let ptr = self.read(ptrptr as u16) as u16;
                let low = self.read(ptr) as u16;
                let high = self.read(ptr.wrapping_add(1)) as u16;
                (high << 8) | low
            }
            AddressingMode::Indirect => {
                let ptr = self.read_u16();
                let low = self.read(ptr) as u16;
                let high = self.read(ptr.wrapping_add(1)) as u16;
                (high << 8) | low
            }
        }
    }
    /// Read the value at the address specified by the addressing mode and the program counter
    pub fn read_addr(&mut self, admod: AddressingMode) -> u8 {
        match admod {
            AddressingMode::Immediate => self.read_opbyte(),
            _ => {
                let addr = self.get_addr(admod);
                self.read(addr)
            }
        }
    }

    pub fn load_pgrm(&mut self, pgrm: Vec<u8>) {
        self.mem.load_pgrm(pgrm);
        self.pc.set_entry_point(PGRM_LOAD_OFFSET);
    }
    // returns a 64 byte slice of the memory centered around the program counter
    pub fn get_pcounter_area(&self) -> Vec<u8> {
        let start = self.pc.read() - 32;
        let end = self.pc.read() + 32;
        self.mem.mem[start as usize..end as usize].iter().map(|x| x.byte).collect::<Vec<u8>>()
    }
}

#[cfg(test)]
mod cpu_tests {
    use super::*;
    #[test]
    fn test_cpu_reset() {
        let mut cpu = CPU::new();
        cpu.a.write(0xFF);
        cpu.x.write(0xFF);
        cpu.y.write(0xFF);
        cpu.pc.write(0xFF);
        cpu.mem.write(0xFF, 0xFF);
        cpu.reset();
        assert_eq!(cpu.a.read(), 0);
        assert_eq!(cpu.x.read(), 0);
        assert_eq!(cpu.y.read(), 0);
        assert_eq!(cpu.pc.read(), 0);
        assert_eq!(cpu.mem.read_u8(0xFF), 0);
    }
}
