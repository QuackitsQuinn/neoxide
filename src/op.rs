use crate::{cpu::CPU, reg::Register};


/// Delegates the execution of the next operation to the appropriate function
pub fn exec_op(cpu: &mut CPU) {
    let op = cpu.read_opbyte();

    match op {
        0x00 => nop(cpu),
        0xA9 => lda::lda_im(cpu),
        0xA5 => lda::lda_zp(cpu),
        0xAD => lda::lda_abs(cpu),
        0xBD => lda::lda_abs_x(cpu),
        0xB9 => lda::lda_abs_y(cpu),
        _ => todo!("Implement op: {:X}", op)
    }

}
/// No op - does nothing
fn nop(cpu: &mut CPU) {
    return;
}

fn read_addr(cpu: &mut CPU) -> u16 {
    let addr1 = cpu.read_opbyte();
    let addr2 = cpu.read_opbyte();
    ((addr2 as u16) << 8) | addr1 as u16
}
mod lda {
    use super::*;

    fn check_zero(cpu: &mut CPU, data: u8) {
        if data == 0 {
            cpu.status.set_zero(true);
        } else {
            cpu.status.set_zero(false);
        }
    }

    fn write_a(cpu: &mut CPU, data: u8) {
        cpu.a.write(data);
        check_zero(cpu, data);
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