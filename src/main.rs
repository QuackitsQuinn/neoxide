#[macro_use] extern crate log;
use std::{panic::{catch_unwind, AssertUnwindSafe}, fs::File, path::{Path}};

use cpu::CPU;
use log::{info, LevelFilter, warn};
use ops::op::exec_op;
use reg::Register;
use simplelog::{TerminalMode, TermLogger, ConfigBuilder};

mod addressing;
mod cpu;
mod cpu_flags;
mod mem_segment;
mod memory;
mod ops;
mod reg;
mod stack;
mod constant;
// no-op then jne to no op
fn main() {
    TermLogger::init(LevelFilter::Debug, ConfigBuilder::new().set_location_level(LevelFilter::Error).build(), TerminalMode::Mixed,simplelog::ColorChoice::Auto).unwrap();
    // this is not how this will work in the future, closer to CPU::new().load_pgrm(vec![0xEA, 0xEA, 0xD0,0xFC,0xFF] or file).run()
    // Because we expect that the cpu will be in an invalid state after a panic
    // we need to wrap it in an AssertUnwindSafe
    let mut cpu = CPU::new();
    let p = AssertUnwindSafe(&mut cpu);
    let result = catch_unwind(|| {
        let inner = p;
        let cpu = inner.0; // very weird way to get cpu in scope
        cpu.load_pgrm(vec![0xEA, 0xEA, 0xD0,0xFC,0xFF]);
        cpu.pc.reset();
        loop {
            //debug!("Registers: A: 0x{:X} X: 0x{:X} Y: 0x{:X} PC: 0x{:X} ({})", cpu.a.read(), cpu.x.read(), cpu.y.read(), cpu.pc.read(), cpu.pc.read());
            exec_op(cpu);
        }
    });
    match result {
        Ok(_) => {
            info!("Closing Neoxide")
        }
        Err(_) => {
            error!("Neoxide has crashed! Printing debug info");
            warn!("Registers: A: 0x{:X} X: 0x{:X} Y: 0x{:X} PC: 0x{:X}", cpu.a.read(), cpu.x.read(), cpu.y.read(), cpu.pc.read());
            warn!("Stack (stack pointer: {}): {:?}", cpu.stack.sp.read(), cpu.stack.stack);
            warn!("Program counter data: {:?}", cpu.get_pcounter_area());
            warn!("Dumping memory to memory_dump.bin");
            let err = cpu.mem.dump(&mut File::create(Path::new("memory_dump.bin")).unwrap());
            if let Err(e) = err {
                error!("Failed to dump memory: {}", e);
            }
        },
    }


    
}
