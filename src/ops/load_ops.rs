use crate::{reg::{Register, U8Register}, cpu::CPU};


fn check_flags(cpu: &mut CPU, data: u8) {
    if data == 0 {
        cpu.status.set_zero(true);
    } else {
        cpu.status.set_zero(false);
    }

    if data & 0x80 == 0x80 {
        cpu.status.set_negative(true);
    } else {
        cpu.status.set_negative(false);
    }
}

pub mod lda {
    use crate::{ops::op::*, cpu::CPU, reg::Register};

    use super::check_flags;

    fn write_a(cpu: &mut CPU, data: u8) {
        cpu.a.write(data);
        check_flags(cpu, data)
    }

    pub fn lda_im(cpu: &mut CPU) {
        let data = cpu.read_opbyte();
        cpu.a.write(data);
        write_a(cpu, data)
    }

    pub fn lda_zp(cpu: &mut CPU) {
        let addr = cpu.read_opbyte();
        let data = cpu.read(addr as u16);
        cpu.a.write(data);
        write_a(cpu, data)
    }

    pub fn lda_abs(cpu: &mut CPU) {
        let addr = read_addr(cpu);
        let data = cpu.read(addr);
        cpu.a.write(data);
        write_a(cpu, data)
    }

    pub fn lda_abs_x(cpu: &mut CPU) {
        let addr = read_addr(cpu);
        let data = cpu.read(addr + cpu.x.read() as u16);
        cpu.a.write(data);
        write_a(cpu, data)
    }

    pub fn lda_abs_y(cpu: &mut CPU) {
        let addr = read_addr(cpu);
        let data = cpu.read(addr + cpu.y.read() as u16);
        cpu.a.write(data);
        write_a(cpu, data)
    }
}

pub mod ldx {
    use crate::{cpu::CPU, reg::Register, ops::op::read_addr};

    fn write(cpu: &mut CPU, data: u8) {
        cpu.x.write(data);
        
    }

    pub fn ldx_im(cpu: &mut CPU) {
        let data = cpu.read_opbyte();
        cpu.x.write(data);
    }

    pub fn ldx_zp(cpu: &mut CPU) {
        let addr = cpu.read_opbyte();
        let data = cpu.read(addr as u16);
        cpu.x.write(data);
    }

    pub fn ldx_zp_y(cpu: &mut CPU) {
        let addr = cpu.read_opbyte();
        let data = cpu.read(addr as u16 + cpu.y.read() as u16);
        cpu.x.write(data);
    }

    pub fn ldx_abs(cpu: &mut CPU) {
        let addr = read_addr(cpu);
        let data = cpu.read(addr);
        cpu.x.write(data);
    }

    pub fn ldx_abs_y(cpu: &mut CPU) {
        let addr = read_addr(cpu);
        let data = cpu.read(addr + cpu.y.read() as u16);
        cpu.x.write(data);
    }
}