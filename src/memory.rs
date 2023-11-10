use crate::constant::PGRM_LOAD_OFFSET;
use core::fmt::Debug;
use std::{
    fs::File,
    io::{self, Write},
};
// TODO: mem mapping
#[derive(Debug)]
pub struct Memory {
    pub mem: [u8; 0xFFFF],
}

impl Memory {
    pub fn new() -> Self {
        Memory { mem: [0; 0xFFFF] }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        self.mem[addr as usize] = value;
    }

    pub fn reset(&mut self) {
        for i in 0..0xFFFF {
            self.mem[i] = 0;
        }
    }

    pub fn load_pgrm(&mut self, pgrm: Vec<u8>) {
        assert!(
            pgrm.len() <= 0xFFFF - PGRM_LOAD_OFFSET as usize,
            "Program too large to fit in memory"
        );
        for (i, byte) in pgrm.iter().enumerate() {
            self.mem[PGRM_LOAD_OFFSET as usize + i] = *byte;
        }
    }

    pub fn dump(&self, file: &mut File) -> Result<(), io::Error> {
        file.write_all(&self.mem)
    }
}
