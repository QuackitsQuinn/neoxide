use crate::{reg::{U8Register, ProgramCounter, Register}, memory::Memory, cpu_flags::CPUStatus, addressing::AddressingMode};


pub struct CPU {
    pub a: U8Register,
    pub x: U8Register,
    pub y: U8Register,
    pub pc: ProgramCounter,
    mem: Memory,
    pub status: CPUStatus
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            a: U8Register::new(0),
            x: U8Register::new(0),
            y: U8Register::new(0),
            pc: ProgramCounter::new(0),
            mem: Memory::new(),
            status: CPUStatus::new()
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

    pub fn read_opbyte(&mut self) -> u8 {
        self.pc.incr();
        self.mem.read_u8(self.pc.read()) // this is a bit weird but it works
    }

    pub fn read_u16(&mut self) -> u16 {
        self.pc.incr();
        let low = self.mem.read_u8(self.pc.read()) as u16;
        self.pc.incr();
        let high = self.mem.read_u8(self.pc.read()) as u16;
        (high << 8) | low
    }

    }

    pub fn read(&mut self, addr: u16) -> u8 {
        self.mem.read_u8(addr)
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.mem.write(addr, data);
    }

    pub fn read_addr(&mut self, admod: AddressingMode) -> u16 {
        match admod {
            AddressingMode::Immediate => self.read_opbyte() as u16,
            AddressingMode::ZeroPage => self.read_opbyte() as u16,
            AddressingMode::ZeroPageX => (self.read_opbyte() as u16).wrapping_add(self.x.read() as u16),
            AddressingMode::ZeroPageY => (self.read_opbyte() as u16).wrapping_add(self.y.read() as u16),
            AddressingMode::Absolute => self.read_u16() as u16,
            AddressingMode::AbsoluteX => (self.read_u16() as u16).wrapping_add(self.x.read()) as u16,
            AddressingMode::AbsoluteY => (self.read_u16() as u16).wrapping_add(self.y.read()) as u16,
            AddressingMode::IndirectX => {
                // src: https://bugzmanov.github.io/nes_ebook/chapter_3_2.html (genuinely have no clue how indirect addressing works)
                let base = self.read_opbyte();

                let ptr: u8 = (base as u8).wrapping_add(self.x.read());
                let lo = self.read(ptr as u16);
                let hi = self.read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::IndirectY => {
                let base = self.read_opbyte();

                let ptr: u8 = (base as u8).wrapping_add(self.y.read());
                let lo = self.read(ptr as u16);
                let hi = self.read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::NoneAddressing => {
                panic!("No addressing mode specified")
            }

        }
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