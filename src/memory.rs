use crate::mem_segment::MemorySegment;
// TODO: mem mapping
pub struct Memory {
    pub mem: [MemorySegment; 0xFFFF],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            mem: [MemorySegment::new(0); 0xFFFF],
        }
    }

    pub fn read(&self, addr: u16) -> MemorySegment {
        self.mem[addr as usize]
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        self.mem[addr as usize].byte
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        self.mem[addr as usize].byte = value;
    }

    pub fn reset(&mut self) {
        for i in 0..0xFFFF {
            self.mem[i].reset();
        }
    }

    pub fn load_pgrm(&mut self, pgrm: Vec<u8>) {
        for (i, byte) in pgrm.iter().enumerate() {
            // TODO: check if this offset is correct
            self.mem[0x2000 + i] = byte.clone().into();
        }   
    }
}
