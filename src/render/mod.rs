use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::rect::Rect;

pub const FONT_BYTES_PER_CHAR: usize = 5;

pub struct ScreenBuffer
{
    pixels: [[bool; 64]; 32],
}

pub struct Texture
{
    rows: Vec<[bool; 8]>,
}

pub struct Sprite {
    x: u8,
    y: u8,
    texture: Texture,
}

pub fn create_sprite(x: u8, y: u8, rows: &Vec<[bool; 8]>) -> Sprite {
    let texture = Texture{rows: rows.clone()};
    Sprite{x: x, y: y, texture: texture}
}

pub fn create_screen_buffer() -> ScreenBuffer {
    let screenBuffer = ScreenBuffer{pixels: [[false; 64]; 32]};
    return screenBuffer;
}

pub fn blit_texture(screenBuffer: &mut ScreenBuffer, sprite: &Sprite) -> bool {
    let mut flipped = false;
    for row in &sprite.texture.rows {
        for x in 0..8 {
            flipped |= row[x] & screenBuffer.pixels[sprite.y as usize][sprite.x as usize + x];
            screenBuffer.pixels[sprite.y as usize][sprite.x as usize + x] =
                row[x] ^ screenBuffer.pixels[sprite.y as usize][sprite.x as usize + x];
        }
    }

    return flipped;
}

pub fn clear_screen(screenBuffer: &mut ScreenBuffer) {
    screenBuffer.pixels = [[false; 64]; 32];
}

pub fn render(canvas: &mut Canvas<sdl2::video::Window>, screenBuffer: &ScreenBuffer) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(0, 255, 0));
    let PIXEL_SIZE: usize = 10;
    for y in 0..32  {
        for x in 0..64 {
            if screenBuffer.pixels[y][x] {
                canvas.fill_rect(Rect::new(PIXEL_SIZE as i32, PIXEL_SIZE as i32, (x * PIXEL_SIZE) as u32, (y * PIXEL_SIZE) as u32));
            }
        }
    }

    canvas.present();
}
