#![allow(dead_code)]

extern crate rand;
extern crate sdl2;

mod machine;
mod opcode;
mod operations;
mod render;

use std::time::{Duration, Instant};

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

mod load {
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

use machine;

pub fn read(mach: &mut machine::Machine, filename: &String) {
    let mut f = File::open(filename).unwrap();
    let mut buffer: [u8; machine::MEM_SIZE] = [0; machine::MEM_SIZE];
    f.read(&mut buffer);

    for i in 0..(machine::MEM_SIZE - machine::START_USER_SPACE) {
        machine::write_memory(mach, machine::START_USER_SPACE + i, buffer[i]);
    }
}
}

pub fn main() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();


    let window = video_subsystem.window("chip8 emulator", 640, 320)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

   
    let mut machine = machine::create_machine();
    let filename = String::from("png.ch8");
    load::read(&mut machine, &filename);


    let op_code_lib = opcode::create_op_code_lib();

    let mut dt: Duration = Duration::from_millis(4);
    let sixty_hz_time = Duration::from_millis(16);

    let key_map = [sdl2::keyboard::Scancode::A,
                   sdl2::keyboard::Scancode::Z,
                   sdl2::keyboard::Scancode::S,
                   sdl2::keyboard::Scancode::X,
                   sdl2::keyboard::Scancode::D,
                   sdl2::keyboard::Scancode::C,
                   sdl2::keyboard::Scancode::F,
                   sdl2::keyboard::Scancode::V,
                   sdl2::keyboard::Scancode::J,
                   sdl2::keyboard::Scancode::N,
                   sdl2::keyboard::Scancode::K,
                   sdl2::keyboard::Scancode::M,
                   sdl2::keyboard::Scancode::L,
                   sdl2::keyboard::Scancode::Comma,
                   sdl2::keyboard::Scancode::Semicolon,
                   sdl2::keyboard::Scancode::Period];

    let mut dt_t = Duration::from_millis(0);
    let mut stopped = false;
    loop {
        for event in event_pump.poll_iter() {}

        let start_time = Instant::now();
        let pc = machine::get_program_counter(&machine);

        // update key state
        for k in 0..16 {
            let pressed = event_pump.keyboard_state().is_scancode_pressed(key_map[k]);
            machine::set_key(&mut machine, k, pressed);
        }

        // run timers
        dt_t += dt;
        while dt_t > sixty_hz_time {
            let mut delay_timer = machine::get_delay_timer(&machine);
            if delay_timer > 0 {
                delay_timer -= 1;
            }
            machine::set_delay_timer(&mut machine, if delay_timer > 0 { delay_timer } else { 0 });

            let mut sound_timer = machine::get_sound_timer(&machine);
            if sound_timer > 0 {
                sound_timer -= 1;
            }
            machine::set_sound_timer(&mut machine, if sound_timer > 0 { sound_timer } else { 0 });

            dt_t -= sixty_hz_time;
        }

        let opcode_part_a = machine::read_memory(&machine, pc);
        let opcode_part_b = machine::read_memory(&machine, pc + 1);
        
        let opcode = opcode::create_opcode(opcode_part_a, opcode_part_b, &op_code_lib);
        //println!("program counter: {:X}", pc);
        //println!("opcode: {:X}, index: {}", opcode.raw, opcode.operation_index);
        
        //println!("target register: {:X}", machine::get_target_register(&machine));
        opcode::execute_opcode(&opcode, &mut machine);
        //println!("address register: {:X}", machine::get_address_register(&machine));
        //for j in 0..machine::NUM_REGISTERS {
        //    println!("V{:X}: {:X}", j, machine::get_register(&machine, j));
        //}
        
        render::render(&mut canvas, machine::get_screenbuffer(&mut machine));

        //machine::increment_program_counter(&mut machine);
        dt = start_time.elapsed();

        //if opcode.operation_index == 23  || stopped {
        if false {
            stopped = true;
            let mut done = false;
            while !done {
                for event in event_pump.poll_iter() {
                    match event {

                        Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => {
                            done = true;
                            break;

                        },

                        _ => {}

                    }
                }
            }
        }
    }

}