extern crate rand;
extern crate sdl2;

mod machine;
mod opcode;
mod operations;
mod render;
mod screen_buffer;

use std::env;
use std::time::{Duration, Instant};
//use std::thread::{sleep};

mod load {
    use std::fs::File;
    use std::io::prelude::*;

    use machine;

    pub fn read(mach: &mut machine::Machine, filename: &String) -> bool {
        let mut f = File::open(filename).unwrap();
        let mut _buffer: [u8; machine::MEM_SIZE] = [0; machine::MEM_SIZE];
        let success = f.read(&mut _buffer);
        match success {
            Ok(_buffer) => (),
            _ => return false,
        }

        for i in 0..(machine::MEM_SIZE - machine::START_USER_SPACE) {
            mach.write_memory(machine::START_USER_SPACE + i, _buffer[i]);
        }

        true
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: chip8 <rom file>");
        return;
    }

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("chip8 emulator", 640, 320)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut machine = machine::Machine::new();
    let filename = String::from(args[1].clone());

    println!("loading {}", filename);
    let loaded = load::read(&mut machine, &filename);
    if loaded {
        println!("ROM loaded");
    } else {
        println!("Failed to load ROM");
        return;
    }

    let op_code_lib = opcode::create_op_code_lib();

    let sixty_hz_time = Duration::from_millis(25); // fudge the timing so that it's not too fast

    let key_map = [
        sdl2::keyboard::Scancode::A,
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
        sdl2::keyboard::Scancode::Period,
    ];

    let mut timer = Instant::now();
    loop {
        for _event in event_pump.poll_iter() {}
        if event_pump
            .keyboard_state()
            .is_scancode_pressed(sdl2::keyboard::Scancode::Escape)
        {
            return;
        }

        // update key state
        for k in 0..16 {
            let pressed = event_pump.keyboard_state().is_scancode_pressed(key_map[k]);
            if pressed && machine.get_flag() == machine::Flags::WaitingForKeypress {
                let target_register = machine.get_target_register();
                machine.set_register(target_register, k as u8);
                machine.set_flag(machine::Flags::Running);
            }
            machine.set_key(k, pressed);
        }

        if machine.get_flag() == machine::Flags::WaitingForKeypress {
            continue;
        }

        // run timers
        let dt = timer.elapsed();
        if dt > sixty_hz_time {
            let mut delay_timer = machine.get_delay_timer();
            if delay_timer > 0 {
                delay_timer -= 1;
            }
            machine.set_delay_timer(if delay_timer > 0 { delay_timer } else { 0 });

            let mut sound_timer = machine.get_sound_timer();
            if sound_timer > 0 {
                sound_timer -= 1;
            }
            machine.set_sound_timer(if sound_timer > 0 { sound_timer } else { 0 });

            timer = Instant::now();
        }

        let pc = machine.get_program_counter();
        let opcode_part_a = machine.read_memory(pc);
        let opcode_part_b = machine.read_memory(pc + 1);

        opcode::decode_and_execute(&mut machine, opcode_part_a, opcode_part_b, &op_code_lib);

        render::render(&mut canvas, machine.get_screenbuffer());
        //std::thread::sleep(Duration::from_millis(1));
    }
}
