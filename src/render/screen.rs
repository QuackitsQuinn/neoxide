
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

    fn update(&mut self, pixeldata: &[u8]) {
        let mut off = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = pixeldata[(y * self.width + x) as usize];
                let (r,g,b) = self.palette[min(pixel, 62) as usize].rgb();
                self.screen_data[off] = r;
                self.screen_data[off + 1] = g;
                self.screen_data[off + 2] = b;
                off += 3;
            }
        }
        // figure out how to draw the screen
    }

    pub fn should_update(&mut self, pixeldata: &[u8]) -> bool {
        let mut off = 0;
        for px in pixeldata {
            if *px != self.last_data[off] {
                self.last_data[off] = *px;
                return true;
            }
            off += 1;
        }
        false
    }

    pub fn render_on_update(&mut self, text: &mut Texture, pixeldata: &[u8]) {
        self.update(pixeldata);
        if self.should_update(pixeldata) {
            text.update(Rect::new(0,0,self.width,self.height), &self.screen_data, (self.width * 3) as usize).unwrap();
        }
    }
}

