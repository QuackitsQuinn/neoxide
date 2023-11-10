use std::fs::File;

use rand::Rng;
use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum, EventPump, Sdl};

use crate::{
    constant::{SCREEN_DATA_OFFSET, TEXTURE_SIZE, WINDOW_SIZE},
    cpu::CPU,
    ops::op::exec_op,
    render::screen::Screen,
};
/// Create a texture target with the specified format, width, and height
macro_rules! create_texture_target {
    ($texture_creator:expr, $format:expr, $width:expr, $height:expr) => {{
        let texture = $texture_creator
            .create_texture_target($format, $width, $height)
            .unwrap();
        texture
    }};
}
/// Get a slice of the screen data from the memory
macro_rules! get_screen_slice {
    ($mem:expr) => {
        &$mem[SCREEN_DATA_OFFSET..SCREEN_DATA_OFFSET + (TEXTURE_SIZE.0 * TEXTURE_SIZE.1) as usize]
    };
}

pub struct NES {
    cpu: CPU,
    screen: Screen,
    // keep sdl so we can do event stuff
    sdl: sdl2::Sdl,
    // TODO: other event handling stuff
}

impl NES {
    /// Create a new NES.
    pub fn new(sdl: Sdl) -> Self {
        let screen = Screen::new(TEXTURE_SIZE.0, TEXTURE_SIZE.1, None);
        NES {
            sdl,
            cpu: CPU::new(),
            screen,
        }
    }
    // because we currently dont have any .rom loading, we just load a raw array of 6502 pgrm bytes
    pub fn load_raw(&mut self, data: &[u8]) {
        self.cpu.load_array(data);
    }
    #[inline(always)]
    pub fn nop(_cpu: &mut CPU) {}
    /// Run the NES with an optional callback.
    /// The callback is called every frame, and is passed a mutable reference to the CPU.
    /// If no callback is provided, a no-op is used.
    /// The callback is intended to be used for a future debugger.
    pub fn run(&mut self, callback: Option<fn(&mut CPU)>) {
        // Create the video subsystem
        let video = self.sdl.video().unwrap();
        // Create a window
        let window = video
            .window("neoxide", WINDOW_SIZE.0, WINDOW_SIZE.1)
            .position_centered()
            .build()
            .expect("Unable to create window");
        // either call the callback or use inlined nop for preformance
        let cb = callback.unwrap_or(NES::nop);
        // Create the canvas and texture (rust is slowly forcing me to use macros)
        let mut canvas = window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .unwrap();
        let texutre_creator = canvas.texture_creator();
        let mut texture = create_texture_target!(
            texutre_creator,
            PixelFormatEnum::RGB24,
            TEXTURE_SIZE.0,
            TEXTURE_SIZE.1
        );
        let mut event_pump = self.sdl.event_pump().unwrap();
        let mut random = rand::thread_rng();
        'running: loop {
            if self.handle_events(&mut event_pump) {
                break 'running;
            }
            self.cpu.write(0xfe, random.gen_range(1..16));
            if !exec_op(&mut self.cpu) {
                break 'running;
            }
            // cb gets called before render for debug purposes
            cb(&mut self.cpu);
            let pixeldata = get_screen_slice!(self.cpu.mem.mem);
            //if self.screen.should_update(pixeldata) {
            self.screen.update(pixeldata);
            self.screen.render(&mut texture, pixeldata);
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
            //}
        }
        self.cpu.mem.dump(&mut File::create("memdump.bin").unwrap());
    }
    /// Handle events from the event pump
    #[inline(always)]
    fn handle_events(&mut self, pump: &mut EventPump) -> bool {
        let mut should_quit = false;
        for event in pump.poll_iter() {
            should_quit = match event {
                Event::Quit { .. } => true,
                _ => false,
            };
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => self.cpu.write(0xff, 0x77),
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => self.cpu.write(0xff, 0x73),
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => self.cpu.write(0xff, 0x61),
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => self.cpu.write(0xff, 0x64),
                _ => {}
            }
        }
        should_quit
    }
}
