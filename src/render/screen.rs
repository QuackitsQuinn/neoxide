
use std::cmp::min;

use sdl2::{Sdl, video::{Window, WindowContext}, render::{Canvas, Texture, TextureCreator}, pixels::{PixelFormatEnum, Color}, rect::Rect};

use super::pal::Palette;




/// A struct representing a screen.
/// Stores the width and height of the screen, as well as the pixel memory as a direct reference to the array.
/// This is done to avoid copying the array, which would be computationally expensive, so we just store a reference to it.
pub struct Screen {
    pub width: u32,
    pub height: u32,
    last_data: Vec<u8>,
    screen_data: Vec<u8>,
    palette: [Color; 64],

}

impl Screen {
    /// Create a new screen with the specified width and height
    pub fn new(width: u32, height: u32, palette: Option<Palette>) -> Screen {
        Screen {
            width,
            height,
            last_data: vec![0; (width * height) as usize],
            screen_data: vec![0; (width * height * 3) as usize],
            palette: palette.unwrap_or_default().colors,
        }
        
        }
    /// Update the screen with the specified pixel data
    pub fn update(&mut self, pixeldata: &[u8]) {
        let mut off = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = pixeldata[(y * self.width + x) as usize];
                let (r,g,b) = self.palette[min(pixel, 63) as usize].rgb();
                self.screen_data[off] = r;
                self.screen_data[off + 1] = g;
                self.screen_data[off + 2] = b;
                off += 3;
            }
        }
        // figure out how to draw the screen
    }

    pub fn should_update(&mut self, pixeldata: &[u8]) -> bool {
        // soo idk how fast build in equality checking is, but this is probably faster than what i would write
        if self.last_data == pixeldata {
            self.last_data = pixeldata.to_vec();
            return false
        }
        true
    }

    pub fn render(&mut self, text: &mut Texture, pixeldata: &[u8]) {
        text.update(Rect::new(0,0,self.width,self.height), &self.screen_data, (self.width * 3) as usize).unwrap();
    }

    pub fn render_on_update(&mut self, text: &mut Texture, pixeldata: &[u8]) {
        if self.should_update(pixeldata) {
            text.update(Rect::new(0,0,self.width,self.height), &self.screen_data, (self.width * 3) as usize).unwrap();
        }
    }
}

