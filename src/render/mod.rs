use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;

use screen_buffer;

pub fn render(
    canvas: &mut Canvas<sdl2::video::Window>,
    screen_buffer: &screen_buffer::ScreenBuffer,
) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(0, 255, 0));
    let pixel_size: usize = 10;
    for y in 0..screen_buffer::SCREEN_HEIGHT {
        for x in 0..screen_buffer::SCREEN_WIDTH {
            if screen_buffer.get_pixels()[y][x] {
                let _result = canvas.fill_rect(Rect::new(
                    (x * pixel_size) as i32,
                    (y * pixel_size) as i32,
                    pixel_size as u32,
                    pixel_size as u32,
                ));
                // TODO: check for errors
            }
        }
    }

    canvas.present();
}
