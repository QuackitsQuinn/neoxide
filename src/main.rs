#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;


use std::{
    fs::File,
    panic::{catch_unwind, AssertUnwindSafe},
    path::Path,
    process::exit,
};

use cpu::CPU;
use log::{info, warn, LevelFilter};
use nes::NES;
use ops::op::exec_op;
use reg::Register;
use render::screen::Screen;
use sdl2::{pixels::Color, event::Event, keyboard::Keycode};
use sdl2::pixels::PixelFormatEnum::RGB24;
use simplelog::{ConfigBuilder, TermLogger, TerminalMode, CombinedLogger};

mod addressing;
mod constant;
mod cpu;
mod cpu_flags;
mod memory;
mod ops;
mod reg;
mod stack;
mod render;
mod nes;
// no-op then jne to no op
fn log(cpu: &mut CPU) {
    info!("A: {:02X} X: {:02X} Y: {:02X} PC: {:04X} SP: {:02X} Flags: {}", cpu.a.read(), cpu.x.read(), cpu.y.read(), cpu.pc.read(), cpu.stack.peek(), cpu.status);
}
fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(
                LevelFilter::Trace,
                ConfigBuilder::new()
                    .set_location_level(LevelFilter::Warn)
                    .build(),
                TerminalMode::Mixed,
                simplelog::ColorChoice::Auto,
            ),
            simplelog::WriteLogger::new(
                LevelFilter::Trace,
                ConfigBuilder::new()
                    .set_location_level(LevelFilter::Warn)
                    .build(),
                File::create("neoxide.log").unwrap(),
            ),
        ]
    ).unwrap();
    let sdl = sdl2::init().unwrap();

    let mut nes = NES::new(sdl);
    //load a modified snake rom that just busy waits when the snake dies
    nes.load_raw(include_bytes!("../res/snake.bin"));

    nes.run(None);

}
/// Handles a crash by printing out the state of the CPU and dumping memory to a file
fn handle_crash(cpu: &mut CPU) {
    eprintln!("Neoxide has crashed!");

    println!("========= CRASH REPORT =========");
    println!();

    println!(
        "========= REGISTERS ========= \n A: {:#X} \n X: {:#X}  \n Y: {:#X} \n PC: {:#X}",
        cpu.a.read(),
        cpu.x.read(),
        cpu.y.read(),
        cpu.pc.read()
    );
    println!("========= STACK =========");
    println!("Stack pointer: {:#X}", cpu.stack.sp.read());
    println!("Stack data:");
    for i in 0..0x100 {
        if (i) % 16 == 0 && i != 0 {
            println!();
            print!("{:#04X} ", i);
        }
        print!("{:#X} ", cpu.stack.stack[i]);
    }
    println!();
    println!("========= PROGRAM COUNTER =========");
    let pg_area = cpu.get_pcounter_area();
    let area = pg_area.0;
    let off = pg_area.1;
    let mut slice_before: Vec<u8> = vec![];
    if off == 0 {
        println!("Program counter is at the start of memory");
    } else if off == 0xFFFF {
        println!("Program counter is at the end of memory");
        slice_before = area[(0..(off - 1) as usize)].to_vec();
    } else {
        println!(
            "Program counter is at offset {:#X} from the start of the memory dump",
            off
        );
        slice_before = area[(0..(off - 1) as usize)].to_vec();
    }
    let slice_after = &area[((off + 1) as usize)..].to_vec();
    println!("Program counter area:");
    print!("{:?}", slice_before);
    print!(" {:#X} (pc) ", area[off as usize]);
    println!("{:?}", slice_after);

    println!("========= CPU FLAGS =========");
    println!("{}", cpu.status);
    println!("Dumping memory to memory_dump.bin");
    let err = cpu
        .mem
        .dump(&mut File::create(Path::new("memory_dump.bin")).unwrap());
    if let Err(e) = err {
        eprintln!("Failed to dump memory: {}", e);
    } else {
        println!("Memory dumped to memory_dump.bin");
    }

    println!("Debug info has been printed; exiting");

    exit(-i32::MAX);
}
