
use sdl2::video::Window;

use crate::{cpu::CPU, render::screen::Screen};





pub struct NES {
    cpu: CPU,
    screen: Screen,
    window: Window,
    // keep sdl so we can do event stuff
    sdl: sdl2::Sdl,
    // TODO: other event handling stuff
}

impl NES {
    pub fn new() -> Self {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let window = video
            .window("neoxide", 256, 240)
            .position_centered()
            .build()
            .unwrap();
        let screen = Screen::new(256, 240, None);
        NES {
            sdl,
            cpu: CPU::new(),
            screen,
            window,
        }
    }
}
