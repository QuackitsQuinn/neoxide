use crate::{addressing::AddressingMode, cpu::CPU, reg::Register};

use super::op::check_flags;

pub fn ora(cpu: &mut CPU, mode: AddressingMode) {
    let data = cpu.read_addr(mode);
    cpu.a.write(cpu.a.read() | data);
    check_flags(cpu, data);
}

pub fn and(cpu: &mut CPU, mode: AddressingMode) {
    let data = cpu.read_addr(mode);
    cpu.a.write(cpu.a.read() & data);
    check_flags(cpu, data);
}

// L imagine using eor instead of xor
pub fn eor(cpu: &mut CPU, mode: AddressingMode) {
    let data = cpu.read_addr(mode);
    cpu.a.write(cpu.a.read() ^ data);
    check_flags(cpu, data);
}

pub fn adc(cpu: &mut CPU, mode: AddressingMode) {
    let data = cpu.read_addr(mode);
    let a = cpu.a.read();
    let (result, overflow) = a.overflowing_add(data);
    cpu.a.write(result);
    check_flags(cpu, result);
    cpu.status.set_overflow(overflow);
}

pub fn sbc(cpu: &mut CPU, mode: AddressingMode) {
    let data = cpu.read_addr(mode);
    let a = cpu.a.read();
    let (result, overflow) = a.overflowing_sub(data);
    cpu.a.write(result);
    check_flags(cpu, result);
    cpu.status.set_overflow(overflow);
}

fn exec_compare(cpu: &mut CPU, data: u8, cmp: u8) {
    if cmp >= data {
        cpu.status.set_carry(true);
    } else {
        cpu.status.set_carry(false);
    }
    check_flags(cpu, cmp.wrapping_sub(data));
}

pub fn cmp(cpu: &mut CPU, mode: AddressingMode) {
    let data = cpu.read_addr(mode);
    exec_compare(cpu, data, cpu.a.read());
}

pub fn cpx(cpu: &mut CPU, mode: AddressingMode) {
    let data = cpu.read_addr(mode);
    exec_compare(cpu, data, cpu.x.read());
}

pub fn cpy(cpu: &mut CPU, mode: AddressingMode) {
    let data = cpu.read_addr(mode);
    exec_compare(cpu, data, cpu.y.read());
}

pub fn bit(cpu: &mut CPU, mode: AddressingMode) {
    let data = cpu.read_addr(mode);
    let a = cpu.a.read();
    cpu.status.set_zero(a & data == 0);
    cpu.status.set_overflow(data & 0b0100_0000 != 0);
    cpu.status.set_negative(data & 0b1000_0000 != 0);
}
// arithmetic shift left
pub fn asl(cpu: &mut CPU, mode: AddressingMode) {
    let data = match mode {
        AddressingMode::Implied => {
            let a = cpu.a.read();
            cpu.status.set_carry(a & 0b1000_0000 != 0);
            cpu.a.write(a << 1);
            a << 1
        }
        _ => {
            let addr = cpu.get_addr(mode);
            let data = cpu.read_addr(mode);
            cpu.status.set_carry(data & 0b1000_0000 != 0);
            cpu.write(addr, data);
            data << 1
        }
    };
    check_flags(cpu, data);
}
// logical shift right 
pub fn lsr(cpu: &mut CPU, mode: AddressingMode) {
    let data = match mode {
        AddressingMode::Implied => {
            let a = cpu.a.read();
            cpu.status.set_carry(a & 0b0000_0001 != 0);
            cpu.a.write(a >> 1);
            a >> 1
        }
        _ => {
            let addr = cpu.get_addr(mode);
            let data = cpu.read_addr(mode);
            cpu.status.set_carry(data & 0b0000_0001 != 0);
            cpu.write(addr, data);
            data >> 1
        }
    };
    check_flags(cpu, data);
}

pub fn ror(cpu: &mut CPU, mode: AddressingMode) {
    let data = match mode {
        AddressingMode::Implied => {
            let a = cpu.a.read();
            let carry = cpu.status.is_carry();
            cpu.status.set_carry(a & 0b0000_0001 != 0);
            cpu.a.write((a >> 1) | (carry as u8) << 7);
            (a >> 1) | (carry as u8) << 7
        }
        _ => {
            let addr = cpu.get_addr(mode);
            let data = cpu.read_addr(mode);
            let carry = cpu.status.is_carry();
            cpu.status.set_carry(data & 0b0000_0001 != 0);
            cpu.write(addr, data);
            (data >> 1) | (carry as u8) << 7
        }
    };
    check_flags(cpu, data);
}

pub fn rol(cpu: &mut CPU, mode: AddressingMode) {
    let data = match mode {
        AddressingMode::Implied => {
            let a = cpu.a.read();
            let carry = cpu.status.is_carry();
            cpu.status.set_carry(a & 0b1000_0000 != 0);
            cpu.a.write((a << 1) | carry as u8);
            (a << 1) | carry as u8
        }
        _ => {
            let addr = cpu.get_addr(mode);
            let data = cpu.read_addr(mode);
            let carry = cpu.status.is_carry();
            cpu.status.set_carry(data & 0b1000_0000 != 0);
            cpu.write(addr, data);
            (data << 1) | carry as u8
        }
    };
    check_flags(cpu, data);
}

pub fn inc(cpu: &mut CPU, mode: AddressingMode) {
    let addr = cpu.get_addr(mode);
    let data = cpu.read_addr(mode);
    cpu.write(addr, data.wrapping_add(1));
    check_flags(cpu, data.wrapping_add(1));
}

pub fn dec(cpu: &mut CPU, mode: AddressingMode) {
    let addr = cpu.get_addr(mode);
    let data = cpu.read_addr(mode);
    cpu.write(addr, data.wrapping_sub(1));
    check_flags(cpu, data.wrapping_sub(1));
}