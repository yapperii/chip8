extern crate rand;
extern crate sdl2;

mod machine;
mod opcode;
mod operations;


use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;


pub fn main() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();


    let window = video_subsystem.window("rust-sdl2 demo: Video", 640, 320)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let SQUARE_SIZE = 10;
    canvas.set_draw_color(Color::RGB(0, 255, 0));
    canvas.fill_rect(Rect::new(SQUARE_SIZE, SQUARE_SIZE, 10, 10));

    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();



    'running: loop {

        for event in event_pump.poll_iter() {

            match event {

                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {

                    break 'running

                },

                _ => {}

            }

        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        // The rest of the game loop goes here...

    }

}