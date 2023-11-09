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
use ops::op::exec_op;
use reg::Register;
use render::screen::Screen;
use sdl2::{pixels::Color, event::Event, keyboard::Keycode};
use sdl2::pixels::PixelFormatEnum::RGB24;
use simplelog::{ConfigBuilder, TermLogger, TerminalMode};

mod addressing;
mod constant;
mod cpu;
mod cpu_flags;
mod mem_segment;
mod memory;
mod ops;
mod reg;
mod stack;
mod render;
// no-op then jne to no op
fn main() {
    TermLogger::init(
        LevelFilter::Trace,
        ConfigBuilder::new()
            .set_location_level(LevelFilter::Warn)
            .build(),
        TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .unwrap();

    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    // this is not how this will work in the future, closer to CPU::new().load_pgrm(vec![0xEA, 0xEA, 0xD0,0xFC,0xFF] or file).run()
    // Because we expect that the cpu will be in an invalid state after a panic
    // we need to wrap it in an AssertUnwindSafe
    let mut cpu = CPU::new();
    let p = AssertUnwindSafe(&mut cpu);
    // very blanket catch all, but we want debug info, and it prints the panic message anyways
    let result = catch_unwind(|| {
        let inner = p;
        let cpu = inner.0; // very weird way to get cpu in scope
                           //cpu.load_pgrm(vec![0xEA, 0xEA, 0xD0, 0xFC, 0xFF]);
        let snake = include_bytes!("../res/snake.bin");
        let mut pal = vec![];
        // this is **very** temporary. just to throw together a quick demo
        for i in 0..0x3f {
            pal.push(Color::RGB(i*2, i*2 , i*2));
        }
        let window = video
            .window("Neoxide", 512, 512)
            .position_centered()
            .build()
            .unwrap();
        
        let mut canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_target(RGB24, 32, 32).unwrap();
        let mut out = Screen::new(32, 32, pal);
        let mut pump = sdl.event_pump().unwrap();
        cpu.load_array(snake); // intentionally invalid opcodes to test panic
        cpu.pc.reset();
        'mainloop: loop {
            exec_op(cpu);
            out.render_on_update(&mut texture, &cpu.mem.mem[0x0200..0x0200 + 32 * 32]);
            for event in pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'mainloop
                    },
                    _ => {}
                }
            }
            cpu.write(0xfe, rand::random());
            canvas.copy(&texture, None, None).expect("Render failed");
            canvas.present();
            // sleep
            //std::thread::sleep(std::time::Duration::from_millis(1));
        }
        cpu.mem.dump(&mut File::create("dump.bin").unwrap());
        //panic!();  // intentionally panic to test panic handling
    });
    match result {
        Ok(_) => {
            info!("Closing Neoxide")
        }
        Err(_) => {
            handle_crash(&mut cpu);
        }
    }
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
