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
    pub rows: Vec<[bool; 8]>,
}

pub struct Sprite {
    x: u8,
    y: u8,
    pub texture: Texture,
}

pub fn create_sprite(x: u8, y: u8, rows: &Vec<[bool; 8]>) -> Sprite {
    let texture = Texture{rows: rows.clone()};
    Sprite{x: x, y: y, texture: texture}
}

pub fn create_screen_buffer() -> ScreenBuffer {
    let screen_buffer = ScreenBuffer{pixels: [[false; 64]; 32]};
    return screen_buffer;
}

pub fn blit_texture(screen_buffer: &mut ScreenBuffer, sprite: &Sprite) -> bool {
    let mut flipped = false;
    for row in &sprite.texture.rows {
        for x in 0..8 {
            flipped |= row[x] & screen_buffer.pixels[sprite.y as usize][sprite.x as usize + x];
            screen_buffer.pixels[sprite.y as usize][sprite.x as usize + x] =
                row[x] ^ screen_buffer.pixels[sprite.y as usize][sprite.x as usize + x];
        }
    }

    return flipped;
}

pub fn blit_texture_row(screen_buffer: &mut ScreenBuffer, x: u8, y: u8, row: &[bool; 8]) -> bool {
    let mut flipped = false;
    for i in 0..8 {
        let wx: usize = (x as usize + i) % 64;
        flipped |= row[i] & screen_buffer.pixels[y as usize][wx];
        screen_buffer.pixels[y as usize][wx] =
            row[i] ^ screen_buffer.pixels[y as usize][wx];
        println!("screen buffer[{}][{}] = {}", y, wx as u8, screen_buffer.pixels[y as usize][wx]);
    }
    flipped
}

pub fn clear_screen(screen_buffer: &mut ScreenBuffer) {
    println!("clearing screen");
    screen_buffer.pixels = [[false; 64]; 32];
}

pub fn render(canvas: &mut Canvas<sdl2::video::Window>, screen_buffer: &ScreenBuffer) {
    println!("rendering");
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(0, 255, 0));
    let pixel_size: usize = 10;
    for y in 0..32  {
        for x in 0..64 {
            if screen_buffer.pixels[y][x] {
                //println!("drawing pixel");
                let result = canvas.fill_rect(Rect::new((x * pixel_size) as i32, (y * pixel_size) as i32, pixel_size as u32 - 2, pixel_size as u32 -2));
                // TODO: check for errors
            }
        }
    }

    canvas.present();
}
