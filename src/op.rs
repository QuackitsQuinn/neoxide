use crate::{cpu::CPU, reg::Register};


/// Delegates the execution of the next operation to the appropriate function
pub fn exec_op(cpu: &mut CPU) {
    let op = cpu.read_opbyte();

    match op {
        0x00 => nop(cpu),
        _ => todo!("Implement op: {:X}", op)
    }

}
/// No op - does nothing
fn nop(cpu: &mut CPU) {
    return;
}