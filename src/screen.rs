use std::borrow::BorrowMut;

use sdl2::{Sdl, video::{Window, WindowContext}, render::{Canvas, Texture, TextureCreator}, pixels::{PixelFormatEnum, Color}};



/// A struct representing a screen.
/// Stores the width and height of the screen, as well as the pixel memory as a direct reference to the array.
/// This is done to avoid copying the array, which would be computationally expensive, so we just store a reference to it.
pub struct Screen<'mem>{
    pub width: u32,
    pub height: u32,
    pixel_memory: &'mem [u8],
    screen_data: Vec<u8>,

}

impl Screen<'_> {
    /// Create a new screen with the specified width and height
    pub fn new(width: u32, height: u32, pixel_memory: &[u8], sdl_ctx: Sdl) -> Screen {
        assert!(width > 0 && height > 0, "Width and height must be greater than 0");
        assert!(pixel_memory.len() == (width * height) as usize, "Array length mismatch! Expected {} but got {}", (width * height) as usize, pixel_memory.len());
        Screen {
            width,
            height,
            pixel_memory,
            screen_data: vec![0; (width * height * 3) as usize],
        }
        
        }

    pub fn draw(&mut self) {
        let mut off = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.pixel_memory[(y * self.width + x) as usize];
                let (r,g,b) = get_color(pixel).rgb();
                self.screen_data[off] = r;
                self.screen_data[off + 1] = g;
                self.screen_data[off + 2] = b;
                off += 3;
            }
        }
        // figure out how to draw the screen
    }

    pub fn should_update(&self) -> bool {
        let mut off = 0;
        for px in self.pixel_memory {
            let (r,g,b) = get_color(*px).rgb();
            if self.screen_data[off] != r || self.screen_data[off + 1] != g || self.screen_data[off + 2] != b {
                return true;
            }
            off += 3;
        }
        false
    }
}

fn get_color(col: u8) -> Color {
    match col {
        0 => Color::WHITE,

        _ => Color::BLACK,
    }
}   
