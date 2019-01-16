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
    let filename = String::from("BC_test.ch8");
    load::read(&mut machine, &filename);


    let op_code_lib = opcode::create_op_code_lib();

    let mut dt: Duration = Duration::new(0, 0);
    let sixty_hz_time = Duration::from_millis(16);
    loop {
        let start_time = Instant::now();
        let pc = machine::get_program_counter(&machine);

        // update key state

        // run timers
        let mut dt_t = dt;
        while dt_t > sixty_hz_time {
            let delay_timer = machine::get_delay_timer(&machine) - 1;
            machine::set_delay_timer(&mut machine, if delay_timer > 0 { delay_timer } else { 0 });

            let sound_timer = machine::get_sound_timer(&machine) - 1;
            machine::set_sound_timer(&mut machine, if sound_timer > 0 { sound_timer } else { 0 });

            dt_t -= sixty_hz_time;
        }

        let opcode_part_a = machine::read_memory(&machine, pc);
        let opcode_part_b = machine::read_memory(&machine, pc + 1);
        
        let opcode = opcode::create_opcode(opcode_part_a, opcode_part_b, &op_code_lib);
        println!("program counter: {:X}", machine::get_program_counter(&machine));
        println!("opcode: {:X}{:X}, combined: {:X}, index: {}", opcode_part_a, opcode_part_b, opcode.raw, opcode.operation_index);
        opcode::execute_opcode(&opcode, &mut machine);
        
        for j in 0..machine::NUM_REGISTERS {
            println!("V{:X}: {:X}", j, machine::get_register(&machine, j));
        }
        
        render::render(&mut canvas, machine::get_screenbuffer(&mut machine));

        machine::increment_program_counter(&mut machine);
        dt = start_time.elapsed();

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