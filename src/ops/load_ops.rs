use crate::{cpu::CPU, addressing::AddressingMode, reg::Register};

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
/// Executes the LDA instruction with the given addressing mode and CPU
pub fn lda(cpu: &mut CPU, mode: AddressingMode) {
    let data = cpu.read_addr(mode);
    cpu.a.write(data);
    check_flags(cpu, data);
}

pub fn ldx(cpu: &mut CPU, mode: AddressingMode) {
    let data = cpu.read_addr(mode);
    cpu.x.write(data);
    check_flags(cpu, data);
}


pub mod ldy {
    use crate::{cpu::CPU, ops::op::read_u16, reg::Register};

    use super::check_flags;

    fn write(cpu: &mut CPU, data: u8) {
        cpu.y.write(data);
        check_flags(cpu, data)
    }

    pub fn ldy_im(cpu: &mut CPU) {
        let data = cpu.read_opbyte();
        cpu.y.write(data);
    }

    pub fn ldy_zp(cpu: &mut CPU) {
        let addr = cpu.read_opbyte();
        let data = cpu.read(addr as u16);
        cpu.y.write(data);
    }

    pub fn ldy_zp_x(cpu: &mut CPU) {
        let addr = cpu.read_opbyte();
        let data = cpu.read(addr as u16 + cpu.x.read() as u16);
        cpu.y.write(data);
    }

    pub fn ldy_abs(cpu: &mut CPU) {
        let addr = read_u16(cpu);
        let data = cpu.read(addr);
        cpu.y.write(data);
    }

    pub fn ldy_abs_x(cpu: &mut CPU) {
        let addr = read_u16(cpu);
        let data = cpu.read(addr + cpu.x.read() as u16);
        cpu.y.write(data);
    }
}
