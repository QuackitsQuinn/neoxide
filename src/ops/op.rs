use crate::{cpu::CPU, reg::Register};

use super::load_ops::{lda, ldx};


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
        0xA2 => ldx::ldx_im(cpu),
        0xA6 => ldx::ldx_zp(cpu),
        0xB6 => ldx::ldx_zp_y(cpu),
        0xAE => ldx::ldx_abs(cpu),
        0xBE => ldx::ldx_abs_y(cpu),
        _ => todo!("Implement op: {:X}", op)
    }

}
/// No op - does nothing
fn nop(cpu: &mut CPU) {
    return;
}

pub fn read_u16(cpu: &mut CPU) -> u16 {
    let addr1 = cpu.read_opbyte();
    let addr2 = cpu.read_opbyte();
    ((addr2 as u16) << 8) | addr1 as u16 
}