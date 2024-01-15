pub const FONT_BYTES_PER_CHAR: usize = 5;
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

pub struct ScreenBuffer
{
    pixels: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

impl ScreenBuffer {
pub fn new() -> Self {
    let screen_buffer = ScreenBuffer{pixels: [[false; SCREEN_WIDTH]; SCREEN_HEIGHT]};
    return screen_buffer;
}

pub fn blit_texture_row(&mut self, x: u8, y: u8, row: &[bool; 8]) -> bool {
    let mut flipped = false;
    if y >= SCREEN_HEIGHT as u8 {
        return flipped
    }
    for i in 0..8 {
        let wx: usize = (x as usize + i) % SCREEN_WIDTH;
        let wy: usize = (y as usize) % SCREEN_HEIGHT;
        flipped |= row[i] & self.pixels[wy][wx];
        self.pixels[wy][wx] =
            row[i] ^ self.pixels[wy][wx];
    }
    flipped
}

pub fn clear_screen(&mut self) {
    self.pixels = [[false; SCREEN_WIDTH]; SCREEN_HEIGHT];
}

pub fn get_pixels(&self) -> &[[bool; SCREEN_WIDTH]; SCREEN_HEIGHT] {
    &self.pixels
}
}
