use crate::{cpu::CPU, reg::Register};

use super::load_ops::{lda, ldx, ldy};


/// Delegates the execution of the next operation to the appropriate function.  
/// This function is here because a 255 line match statement is not very readable to be in cpu.rs
pub fn exec_op(cpu: &mut CPU) {
    let op = cpu.read_opbyte();

    match op {
        0xEA => nop(cpu),
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
        0xA0 => ldy::ldy_im(cpu),
        0xA4 => ldy::ldy_zp(cpu),
        0xB4 => ldy::ldy_zp_x(cpu),
        0xAC => ldy::ldy_abs(cpu),
        0xBC => ldy::ldy_abs_x(cpu),
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