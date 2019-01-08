//extern crate rand;
use rand::prelude::*;
//use rand::Rng;
use machine;
use render;

pub fn op_0NNN(mach: &mut machine::Machine) {
    // probably not needed
}

pub fn op_00E0(mach: &mut machine::Machine) {
    render::clear_screen(machine::get_screenbuffer(mach));
}

pub fn op_00EE(mach: &mut machine::Machine) {
    machine::ret(mach);
}

pub fn op_1NNN(mach: &mut machine::Machine, address: usize) {
    machine::jump(mach, address);
}

pub fn op_2NNN(mach: &mut machine::Machine, address: usize) {
    machine::call(mach, address);
}

pub fn op_3XNN(mach: &mut machine::Machine, x: usize, n: u16) {
    let rx = machine::get_register(mach, x) as u16;
    if rx == n {
        machine::increment_program_counter(mach);
    }
}

pub fn op_4XNN(mach: &mut machine::Machine, x: usize, n: u16) {
    let vx = machine::get_register(mach, x) as u16;
    if vx != n {
        machine::increment_program_counter(mach);
    }
}

pub fn op_5XY0(mach: &mut machine::Machine, x: usize, y: usize) {
    let vx = machine::get_register(mach, x);
    let vy = machine::get_register(mach, y);
    if vx == vy {
        machine::increment_program_counter(mach);
    }
}

pub fn op_6XNN(mach: &mut machine::Machine, x: usize, n: u8) {
    machine::set_register(mach, x, n);
}

pub fn op_7XNN(mach: &mut machine::Machine, x: usize, n: u8) {
    let vx = machine::get_register(mach, x);
    let sum: u16 = vx as u16 + n as u16;
    machine::set_register(mach, x, sum as u8);
}

pub fn op_8XY0(mach: &mut machine::Machine, x: usize, y: usize) {
    let vy = machine::get_register(mach, y);
    machine::set_register(mach, x, vy);
}

pub fn op_8XY1(mach: &mut machine::Machine, x: usize, y: usize) {
    let vx = machine::get_register(mach, x);
    let vy = machine::get_register(mach, y);
    machine::set_register(mach, x, vx | vy);
}

pub fn op_8XY2(mach: &mut machine::Machine, x: usize, y: usize) {
    let vx = machine::get_register(mach, x);
    let vy = machine::get_register(mach, y);
    machine::set_register(mach, x, vx & vy);
}

pub fn op_8XY3(mach: &mut machine::Machine, x: usize, y: usize) {
    let vx = machine::get_register(mach, x);
    let vy = machine::get_register(mach, y);
    machine::set_register(mach, x, vx ^ vy);
}

pub fn op_8XY4(mach: &mut machine::Machine, x: usize, y: usize) {
    let vx = machine::get_register(mach, x);
    let vy = machine::get_register(mach, y);
    let sum: u16 = vx as u16 + vy as u16;
    machine::set_register(mach, x, sum as u8);
    machine::set_register(mach, 0xf, if sum > 0xff { 1 } else { 0 });
}

pub fn op_8XY5(mach: &mut machine::Machine, x: usize, y: usize) {
    let vx = machine::get_register(mach, x);
    let vy = machine::get_register(mach, y);
    let difference: i16 = vx as i16 - vy as i16;
    machine::set_register(mach, x, (0x100 + difference) as u8);
    machine::set_register(mach, 0xf, if difference < 0 { 0 } else { 1 });
}

pub fn op_8XY6(mach: &mut machine::Machine, x: usize) {
    let vx = machine::get_register(mach, x);
    machine::set_register(mach, 0xf, vx & 0x1);
    machine::set_register(mach, x, vx >> 1);
}

pub fn op_8XY7(mach: &mut machine::Machine, x: usize, y: usize) {
    let vx = machine::get_register(mach, x);
    let vy = machine::get_register(mach, y);
    let difference: i16 = vy as i16 - vx as i16;
    machine::set_register(mach, x, (0x100 + difference) as u8);
    machine::set_register(mach, 0xf, if difference < 0 { 0 } else { 1 });
}

pub fn op_8XYE(mach: &mut machine::Machine, x: usize) {
    let vx = machine::get_register(mach, x);
    machine::set_register(mach, 0xf, vx & 0x80);
    machine::set_register(mach, x, vx << 1);
}

pub fn op_9XY0(mach: &mut machine::Machine, x: usize, y: usize) {
    let vx = machine::get_register(mach, x) as u16;
    let vy = machine::get_register(mach, y) as u16;
    if vx != vy {
        machine::increment_program_counter(mach);
    }
}

pub fn op_ANNN(mach: &mut machine::Machine, n: usize) {
    machine::set_address_register(mach, n);
}

pub fn op_BNNN(mach: &mut machine::Machine, n: usize) {
    let v0 = machine::get_register(mach, 0x0);
    let address = n + v0 as usize;
    machine::jump(mach, address);
}

pub fn op_CXNN(mach: &mut machine::Machine, x: usize, n: u8) {
    let r = rand::random::<u8>();
    machine::set_register(mach, x, n & r);
}

pub fn op_DXYN(mach: &mut machine::Machine, x: u8, y: u8, n: u8) {
    let base_address = machine::get_address_register(mach);
    let n_size = n as usize;
    let mut rows: Vec<[bool; 8]> = Vec::with_capacity(n_size);
    for i in 0..n {
        let mem_val = machine::read_memory(mach, base_address + n_size);
        let mut row: [bool; 8] = [false; 8];
        for j in 0..8 {
            row[j] = (mem_val & (1 << j)) != 0;
        }
    }

    let sprite = render::create_sprite(x, y, &rows);
    render::blit_texture(machine::get_screenbuffer(mach), &sprite);
}

pub fn op_EX9E(mach: &mut machine::Machine, x: usize) {
    let vx = machine::get_register(mach, x);
    if machine::get_key(mach, vx as usize) {
        machine::increment_program_counter(mach);
    }
}

pub fn op_EXA1(mach: &mut machine::Machine, x: usize) {
    let vx = machine::get_register(mach, x);
    if !machine::get_key(mach, vx as usize) {
        machine::increment_program_counter(mach);
    }
}

pub fn op_FX07(mach: &mut machine::Machine, x: usize) {
    let delay = machine::get_delay_timer(mach);
    machine::set_register(mach, x, delay);
}

pub fn op_FX0A(mach: &mut machine::Machine, x: usize) {
    machine::set_flag(mach, machine::Flags::WAITING_FOR_KEYPRESS);
    machine::set_target_register(mach, x);
}

pub fn op_FX15(mach: &mut machine::Machine, x: usize) {
    let vx = machine::get_register(mach, x);
    machine::set_delay_timer(mach, vx);
}

pub fn op_FX18(mach: &mut machine::Machine, x: usize) {
    let vx = machine::get_register(mach, x);
    machine::set_sound_timer(mach, vx);
}

pub fn op_FX1E(mach: &mut machine::Machine, x: usize) {
    let address_register = machine::get_address_register(mach);
    let vx = machine::get_register(mach, x);
    machine::set_address_register(mach, address_register + vx as usize);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_00EE() {
        let mut mach = machine::create_machine();
        machine::call(&mut mach, 0x300);
        op_00EE(&mut mach);

        assert_eq!(machine::START_USER_SPACE, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_1NNN() {
        let mut mach = machine::create_machine();
        op_1NNN(&mut mach, 0x300);

        assert_eq!(0x300, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_2NNN() {
        let mut mach = machine::create_machine();
        op_2NNN(&mut mach, 0x300);

        assert_eq!(0x300, machine::get_program_counter(&mach));
        assert_eq!(Some(machine::START_USER_SPACE), machine::peek_stack(&mut mach));
    }

    #[test]
    fn test_op_3XNN_pass() {
        let mut mach = machine::create_machine();
        machine::set_register(&mut mach, 0x0, 0x8);
        op_3XNN(&mut mach, 0x0, 0x8);

        assert_eq!(machine::START_USER_SPACE + 2, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_3XNN_fail() {
        let mut mach = machine::create_machine();
        machine::set_register(&mut mach, 0x0, 0x8);
        op_3XNN(&mut mach, 0x0, 0x1);

        assert_eq!(machine::START_USER_SPACE, machine::get_program_counter(&mach));
    }

     #[test]
    fn test_op_4XNN_pass() {
        let mut mach = machine::create_machine();
        machine::set_register(&mut mach, 0x0, 0x8);
        op_4XNN(&mut mach, 0x0, 0x1);

        assert_eq!(machine::START_USER_SPACE + 2, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_4XNN_fail() {
        let mut mach = machine::create_machine();
        machine::set_register(&mut mach, 0x0, 0x8);
        op_4XNN(&mut mach, 0x0, 0x8);

        assert_eq!(machine::START_USER_SPACE, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_5XY0_pass() {
        let mut mach = machine::create_machine();
        machine::set_register(&mut mach, 0x0, 0x8);
        machine::set_register(&mut mach, 0x1, 0x8);
        op_5XY0(&mut mach, 0x0, 0x1);

        assert_eq!(machine::START_USER_SPACE + 2, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_5XY0_fail() {
        let mut mach = machine::create_machine();
        machine::set_register(&mut mach, 0x0, 0x8);
        machine::set_register(&mut mach, 0x1, 0x7);
        op_5XY0(&mut mach, 0x0, 0x1);

        assert_eq!(machine::START_USER_SPACE, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_6XNN() {
        let mut machine = machine::create_machine();
        op_6XNN(&mut machine, 0x0, 0x8);

        assert_eq!(0x8, machine::get_register(&machine, 0x0));
    }

    #[test]
    fn test_op_7XNN() {
        let mut machine = machine::create_machine();
        machine::set_register(&mut machine, 0x0, 0x4);
        op_7XNN(&mut machine, 0x0, 0x4);

        assert_eq!(0x8, machine::get_register(&machine, 0x0));

        op_7XNN(&mut machine, 0x0, 0xfe);

        assert_eq!(0x6, machine::get_register(&machine, 0x0));
        assert_eq!(0x0, machine::get_register(&machine, 0xf));
    }

    #[test]
    fn test_op_8XY0() {
        let mut machine = machine::create_machine();
        machine::set_register(&mut machine, 0x0, 0x1);
        machine::set_register(&mut machine, 0x1, 0x8);
        op_8XY0(&mut machine, 0x0, 0x1);

        assert_eq!(0x8, machine::get_register(&machine, 0x0));
    }

    #[test]
    fn test_op_8XY1() {
        let mut machine = machine::create_machine();
        machine::set_register(&mut machine, 0x0, 0x4);
        machine::set_register(&mut machine, 0x1, 0x1);
        op_8XY1(&mut machine, 0x0, 0x1);

        assert_eq!(0x5, machine::get_register(&machine, 0x0));

        machine::set_register(&mut machine, 0x0, 0x1);
        op_8XY1(&mut machine, 0x0, 0x1);

        assert_eq!(0x1, machine::get_register(&machine, 0x0));
    }

    #[test]
    fn test_op_8XY2() {
        let mut machine = machine::create_machine();
        machine::set_register(&mut machine, 0x0, 0x4);
        machine::set_register(&mut machine, 0x1, 0x1);
        op_8XY2(&mut machine, 0x0, 0x1);

        assert_eq!(0x0, machine::get_register(&machine, 0x0));

        machine::set_register(&mut machine, 0x0, 0x5);
        op_8XY2(&mut machine, 0x0, 0x1);

        assert_eq!(0x1, machine::get_register(&machine, 0x0));
    }

    #[test]
    fn test_op_8XY3() {
        let mut machine = machine::create_machine();
        machine::set_register(&mut machine, 0x0, 0x4);
        machine::set_register(&mut machine, 0x1, 0x1);
        op_8XY3(&mut machine, 0x0, 0x1);

        assert_eq!(0x5, machine::get_register(&machine, 0x0));

        machine::set_register(&mut machine, 0x0, 0x5);
        op_8XY3(&mut machine, 0x0, 0x1);

        assert_eq!(0x4, machine::get_register(&machine, 0x0));
    }

    #[test]
    fn test_op_8XY4() {
        let mut machine = machine::create_machine();
        machine::set_register(&mut machine, 0x0, 0x8);
        machine::set_register(&mut machine, 0x1, 0x9);
        op_8XY4(&mut machine, 0, 1);

        assert_eq!(0x11, machine::get_register(&machine, 0x0));
        assert_eq!(0x0, machine::get_register(&machine, 0xf));

        machine::set_register(&mut machine, 0x0, 0xf8);
        op_8XY4(&mut machine, 0x0, 0x1);

        assert_eq!(0x1, machine::get_register(&machine, 0x0));
        assert_eq!(0x1, machine::get_register(&machine, 0xf));
    }

    #[test]
    fn test_op_8XY5() {
        let mut machine = machine::create_machine();
        machine::set_register(&mut machine, 0x0, 0x9);
        machine::set_register(&mut machine, 0x1, 0x8);
        op_8XY5(&mut machine, 0x0, 0x1);

        assert_eq!(0x1, machine::get_register(&machine, 0x0));
        assert_eq!(0x1, machine::get_register(&machine, 0xf));

        machine::set_register(&mut machine, 0x0, 0x1);
        op_8XY5(&mut machine, 0x0, 0x1);

        assert_eq!(0xf9, machine::get_register(&machine, 0x0));
        assert_eq!(0x0, machine::get_register(&machine, 0xf));
    }

    #[test]
    fn test_op_8XY6() {
        let mut machine = machine::create_machine();
        machine::set_register(&mut machine, 0x0, 0x3);
        op_8XY6(&mut machine, 0x0);

        assert_eq!(0x1, machine::get_register(&machine, 0xf));
        assert_eq!(0x1, machine::get_register(&machine, 0x0));
    }

    #[test]
    fn test_op_8XY7() {
        let mut machine = machine::create_machine();
        machine::set_register(&mut machine, 0x0, 0x8);
        machine::set_register(&mut machine, 0x1, 0x9);
        op_8XY7(&mut machine, 0x0, 0x1);

        assert_eq!(0x1, machine::get_register(&machine, 0x0));
        assert_eq!(0x1, machine::get_register(&machine, 0xf));

        machine::set_register(&mut machine, 0x0, 0xa);
        op_8XY7(&mut machine, 0x0, 0x1);

        assert_eq!(0xff, machine::get_register(&machine, 0x0));
        assert_eq!(0x0, machine::get_register(&machine, 0xf));
    }

    #[test]
    fn test_op_8XYE() {
        let mut machine = machine::create_machine();
        machine::set_register(&mut machine, 0x0, 0xff);
        op_8XYE(&mut machine, 0x0);

        assert_eq!(0x80, machine::get_register(&machine, 0xf));
        assert_eq!(0xfe, machine::get_register(&machine, 0x0));
    }

    #[test]
    fn test_op_9XY0_pass() {
        let mut mach = machine::create_machine();
        machine::set_register(&mut mach, 0x0, 0x8);
        machine::set_register(&mut mach, 0x1, 0x7);
        op_9XY0(&mut mach, 0x0, 0x1);

        assert_eq!(machine::START_USER_SPACE + 2, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_9XY0_fail() {
        let mut mach = machine::create_machine();
        machine::set_register(&mut mach, 0x0, 0x8);
        machine::set_register(&mut mach, 0x1, 0x8);
        op_9XY0(&mut mach, 0x0, 0x1);

        assert_eq!(machine::START_USER_SPACE, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_ANNN() {
        let mut mach = machine::create_machine();
        op_ANNN(&mut mach, 0x300);

        assert_eq!(0x300, machine::get_address_register(&mach));
    }

    #[test]
    fn test_op_BNNN() {
        let mut mach = machine::create_machine();
        machine::set_register(&mut mach, 0, 0x50);
        op_BNNN(&mut mach, 0x300);

        assert_eq!(0x350, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_EX9E_pass() {
        let mut mach = machine::create_machine();
        machine::set_key(&mut mach, 0, true);
        op_EX9E(&mut mach, 0);

        assert_eq!(machine::START_USER_SPACE + 2, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_EX9E_fail() {
        let mut mach = machine::create_machine();
        op_EX9E(&mut mach, 0);

        assert_eq!(machine::START_USER_SPACE, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_EXA1_pass() {
        let mut mach = machine::create_machine();
        op_EXA1(&mut mach, 0);

        assert_eq!(machine::START_USER_SPACE + 2, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_EXA1_fail() {
        let mut mach = machine::create_machine();
        machine::set_key(&mut mach, 0, true);
        op_EXA1(&mut mach, 0);

        assert_eq!(machine::START_USER_SPACE, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_FX07() {
        let mut mach = machine::create_machine();
        machine::set_delay_timer(&mut mach, 10);
        op_FX07(&mut mach, 10);

        assert_eq!(10, machine::get_delay_timer(&mach));
    }

    #[test]
    fn test_op_FX0A() {
        let mut mach = machine::create_machine();
        op_FX0A(&mut mach, 1);

        assert_eq!(machine::Flags::WAITING_FOR_KEYPRESS, machine::get_flag(&mach));
        assert_eq!(1, machine::get_target_register(&mach));
    }

    #[test]
    fn test_op_FX15() {
        let mut mach = machine::create_machine();
        machine::set_register(&mut mach, 0, 10);
        op_FX15(&mut mach, 0);

        assert_eq!(10, machine::get_delay_timer(&mach));
    }

    #[test]
    fn test_op_FX18() {
        let mut mach = machine::create_machine();
        machine::set_register(&mut mach, 0, 10);
        op_FX18(&mut mach, 0);

        assert_eq!(10, machine::get_sound_timer(&mach));
    }

    #[test]
    fn test_op_FX1E() {
        let mut mach = machine::create_machine();
        machine::set_address_register(&mut mach, 0x9);
        machine::set_register(&mut mach, 0x0, 0x2);
        op_FX1E(&mut mach, 0x0);

        assert_eq!(0xb, machine::get_address_register(&mach));
    }
}