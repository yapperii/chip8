use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::rect::Rect;

pub const FONT_BYTES_PER_CHAR: usize = 5;
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

pub struct ScreenBuffer
{
    pixels: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

pub fn create_screen_buffer() -> ScreenBuffer {
    let screen_buffer = ScreenBuffer{pixels: [[false; SCREEN_WIDTH]; SCREEN_HEIGHT]};
    return screen_buffer;
}

pub fn blit_texture_row(screen_buffer: &mut ScreenBuffer, x: u8, y: u8, row: &[bool; 8]) -> bool {
    let mut flipped = false;
    if y >= SCREEN_HEIGHT as u8 {
        return flipped
    }
    for i in 0..8 {
        let wx: usize = (x as usize + i) % SCREEN_WIDTH;
        let wy: usize = (y as usize) % SCREEN_HEIGHT;
        flipped |= row[i] & screen_buffer.pixels[wy][wx];
        screen_buffer.pixels[wy][wx] =
            row[i] ^ screen_buffer.pixels[wy][wx];
    }
    flipped
}

pub fn clear_screen(screen_buffer: &mut ScreenBuffer) {
    screen_buffer.pixels = [[false; SCREEN_WIDTH]; SCREEN_HEIGHT];
}

pub fn render(canvas: &mut Canvas<sdl2::video::Window>, screen_buffer: &ScreenBuffer) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(0, 255, 0));
    let pixel_size: usize = 10;
    for y in 0..SCREEN_HEIGHT  {
        for x in 0..SCREEN_WIDTH {
            if screen_buffer.pixels[y][x] {
                let _result = canvas.fill_rect(Rect::new((x * pixel_size) as i32, (y * pixel_size) as i32, pixel_size as u32, pixel_size as u32));
                // TODO: check for errors
            }
        }
    }

    canvas.present();
}
