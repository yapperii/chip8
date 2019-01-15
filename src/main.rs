#![allow(dead_code)]

extern crate rand;
extern crate sdl2;

mod machine;
mod opcode;
mod operations;
mod render;


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

    let mut event_pump = sdl_context.event_pump().unwrap();

   

    let mut machine = machine::create_machine();
    let op_code_lib = opcode::create_op_code_lib();


    loop {
        let pc = machine::get_program_counter(&machine);

        // update key state

        // run timers

        let opcode_part_a = machine::read_memory(&machine, pc);
        let opcode_part_b = machine::read_memory(&machine, pc + 1);
        let opcode = opcode::create_opcode(opcode_part_a, opcode_part_b, &op_code_lib);
        opcode::execute_opcode(&opcode, &mut machine);
        
        render::render(&mut canvas, machine::get_screenbuffer(&mut machine));

        machine::increment_program_counter(&mut machine);
    }

}