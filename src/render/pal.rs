use sdl2::pixels::Color;

/// Various things for .pal files and just nes palettes in general





pub struct Palette {
    pub colors: [Color; 64],
}

impl Palette {
    pub fn from_bytes(bytes: &[u8]) -> Palette {
        // allow for pal files with deimphasis colors, even if we don't use them
        assert!(bytes.len() >= 64 * 3, "Palette file too small: {} bytes", bytes.len());
        let mut colors = [Color::RGB(0, 0, 0); 64];
        for i in 0..64 {
            let r = bytes[i * 3];
            let g = bytes[i * 3 + 1];
            let b = bytes[i * 3 + 2];
            colors[i] = Color::RGB(r, g, b);
        }
        Palette { colors }
    }
}

impl Default for Palette {
    /// Reads the default palette from the palette.pal file in the res folder
    /// This pal file is generated with [palgen-persune](https://github.com/Gumball2415/palgen-persune)
    fn default() -> Self {
        Self::from_bytes(include_bytes!("../../res/palette.pal"))
    }
}