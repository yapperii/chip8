//extern crate rand;
use rand::prelude::*;
//use rand::Rng;
use machine;
use render;

pub fn op_0nnn(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    // probably not needed
    machine::increment_program_counter(mach);
}

pub fn op_00e0(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    render::clear_screen(machine::get_screenbuffer(mach));
    machine::increment_program_counter(mach);
}

pub fn op_00ee(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    machine::ret(mach);
}

pub fn op_1nnn(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    machine::jump(mach, n as usize);
}

pub fn op_2nnn(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    machine::call(mach, n as usize);
}

pub fn op_3xnn(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    let vx = machine::get_register(mach, x as usize) as u16;
    if vx == n {
        machine::increment_program_counter(mach);
    }
    machine::increment_program_counter(mach);
}

pub fn op_4xnn(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    let vx = machine::get_register(mach, x as usize) as u16;
    if vx != n {
        machine::increment_program_counter(mach);
    }
    machine::increment_program_counter(mach);
}

pub fn op_5xy0(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    let vx = machine::get_register(mach, x as usize);
    let vy = machine::get_register(mach, y as usize);
    if vx == vy {
        machine::increment_program_counter(mach);
    }
    machine::increment_program_counter(mach);
}

pub fn op_6xnn(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    machine::set_register(mach, x as usize, n as u8);
    machine::increment_program_counter(mach);
}

pub fn op_7xnn(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    let vx = machine::get_register(mach, x as usize);
    let sum: u16 = vx as u16 + n as u16;
    machine::set_register(mach, x as usize, sum as u8);
    machine::increment_program_counter(mach);
}

pub fn op_8xy0(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    let vy = machine::get_register(mach, y as usize);
    machine::set_register(mach, x as usize, vy);
    machine::increment_program_counter(mach);
}

pub fn op_8xy1(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    let vx = machine::get_register(mach, x as usize);
    let vy = machine::get_register(mach, y as usize);
    machine::set_register(mach, x as usize, vx | vy);
    machine::increment_program_counter(mach);
}

pub fn op_8xy2(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    let vx = machine::get_register(mach, x as usize);
    let vy = machine::get_register(mach, y as usize);
    machine::set_register(mach, x as usize, vx & vy);
    machine::increment_program_counter(mach);
}

pub fn op_8xy3(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    let vx = machine::get_register(mach, x as usize);
    let vy = machine::get_register(mach, y as usize);
    machine::set_register(mach, x as usize, vx ^ vy);
    machine::increment_program_counter(mach);
}

pub fn op_8xy4(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    let vx = machine::get_register(mach, x as usize);
    let vy = machine::get_register(mach, y as usize);
    let sum: u16 = vx as u16 + vy as u16;
    machine::set_register(mach, x as usize, sum as u8);
    machine::set_register(mach, 0xf, if sum > 0xff { 1 } else { 0 });
    machine::increment_program_counter(mach);
}

pub fn op_8xy5(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    let vx = machine::get_register(mach, x as usize);
    let vy = machine::get_register(mach, y as usize);
    let difference: i16 = vx as i16 - vy as i16;
    machine::set_register(mach, x as usize, (0x100 + difference) as u8);
    machine::set_register(mach, 0xf, if difference < 0 { 0 } else { 1 });
    machine::increment_program_counter(mach);
}

pub fn op_8xy6(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    let vx = machine::get_register(mach, x as usize);
    machine::set_register(mach, 0xf, vx & 0x1);
    machine::set_register(mach, x as usize, vx >> 1);
    machine::increment_program_counter(mach);
}

pub fn op_8xy7(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    let vx = machine::get_register(mach, x as usize);
    let vy = machine::get_register(mach, y as usize);
    let difference: i16 = vy as i16 - vx as i16;
    machine::set_register(mach, x as usize, (0x100 + difference) as u8);
    machine::set_register(mach, 0xf, if vy > vx { 1 } else { 0 });
    machine::increment_program_counter(mach);
}

pub fn op_8xye(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    let vx = machine::get_register(mach, x as usize);
    machine::set_register(mach, 0xf, if (vx & 0x80) > 0 { 1 } else { 0 });
    machine::set_register(mach, x as usize, vx << 1);
    machine::increment_program_counter(mach);
}

pub fn op_9xy0(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    let vx = machine::get_register(mach, x as usize) as u16;
    let vy = machine::get_register(mach, y as usize) as u16;
    if vx != vy {
        machine::increment_program_counter(mach);
    }
    machine::increment_program_counter(mach);
}

pub fn op_annn(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    machine::set_address_register(mach, n as usize);
    machine::increment_program_counter(mach);
}

pub fn op_bnnn(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    let v0 = machine::get_register(mach, 0x0);
    let address = n as usize + v0 as usize;
    machine::jump(mach, address);
}

pub fn op_cxnn(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    let r = rand::random::<u8>();
    machine::set_register(mach, x as usize, n as u8 & r);
    machine::increment_program_counter(mach);
}

pub fn op_dxyn(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    let vx = machine::get_register(mach, x as usize);
    let vy = machine::get_register(mach, y as usize);
    let base_address = machine::get_address_register(mach);
    let mut flipped = false;
    for i in 0..n {
        let mem_val = machine::read_memory(mach, base_address + i as usize);
        let mut row: [bool; 8] = [false; 8];
        for j in 0..8 {
            row[j] = (mem_val & (1 << (8 - j -1))) != 0;
        }

        flipped |= render::blit_texture_row(machine::get_screenbuffer(mach), vx, vy + i as u8, &row);
    }

    machine::set_register(mach, 0xf, if flipped { 1 } else { 0 });
    machine::increment_program_counter(mach);
}

pub fn op_ex9e(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    let vx = machine::get_register(mach, x as usize);
    if machine::get_key(mach, vx as usize) {
        machine::increment_program_counter(mach);
    }
    machine::increment_program_counter(mach);
}

pub fn op_exa1(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    let vx = machine::get_register(mach, x as usize);
    if !machine::get_key(mach, vx as usize) {
        machine::increment_program_counter(mach);
    }
    machine::increment_program_counter(mach);
}

pub fn op_fx07(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    let delay = machine::get_delay_timer(mach);
    machine::set_register(mach, x as usize, delay);
    machine::increment_program_counter(mach);
}

pub fn op_fx0a(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    machine::set_flag(mach, machine::Flags::WaitingForKeypress);
    machine::set_target_register(mach, x as usize);
    machine::increment_program_counter(mach);
}

pub fn op_fx15(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    let vx = machine::get_register(mach, x as usize);
    machine::set_delay_timer(mach, vx);
    machine::increment_program_counter(mach);
}

pub fn op_fx18(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    let vx = machine::get_register(mach, x as usize);
    machine::set_sound_timer(mach, vx);
    machine::increment_program_counter(mach);
}

pub fn op_fx1e(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    let address_register = machine::get_address_register(mach);
    let vx = machine::get_register(mach, x as usize);
    machine::set_address_register(mach, address_register + vx as usize);
    machine::increment_program_counter(mach);
}

pub fn op_fx29(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    let vx = machine::get_register(mach, x as usize);
    let address = machine::START_FONT + (vx as usize * render::FONT_BYTES_PER_CHAR);
    machine::set_address_register(mach, address);
    machine::increment_program_counter(mach);
}

pub fn op_fx33(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    let vx = machine::get_register(&mach, x as usize);
    let base_address = machine::get_address_register(&mach);
    let ones = vx % 10;
    let tens = (vx / 10) % 10;
    let hundreds = (vx / 100) % 10;
    machine::write_memory(mach, base_address, hundreds);
    machine::write_memory(mach, base_address + 1, tens);
    machine::write_memory(mach, base_address + 2, ones);
    machine::increment_program_counter(mach);
}

pub fn op_fx55(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    let base_address = machine::get_address_register(mach);
    for i in 0..((x + 1) as usize){
        let vi = machine::get_register(mach, i);
        machine::write_memory(mach, base_address + i, vi);
    }
    machine::increment_program_counter(mach);
}

pub fn op_fx65(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    let base_address = machine::get_address_register(mach);
    for i in 0..((x + 1) as usize) {
        let mem = machine::read_memory(mach, base_address + i);
        machine::set_register(mach, i, mem);
    }
    machine::increment_program_counter(mach);
}

pub fn execute_by_index(index: usize, mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    match index {
        0 => op_0nnn(mach, x, y, n),
        1 => op_00e0(mach, x, y, n),
        2 => op_00ee(mach, x, y, n),
        3 => op_1nnn(mach, x, y, n),
        4 => op_2nnn(mach, x, y, n),
        5 => op_3xnn(mach, x, y, n),
        6 => op_4xnn(mach, x, y, n),
        7 => op_5xy0(mach, x, y, n),
        8 => op_6xnn(mach, x, y, n),
        9 => op_7xnn(mach, x, y, n),
        10 => op_8xy0(mach, x, y, n),
        11 => op_8xy1(mach, x, y, n),
        12 => op_8xy2(mach, x, y, n),
        13 => op_8xy3(mach, x, y, n),
        14 => op_8xy4(mach, x, y, n),
        15 => op_8xy5(mach, x, y, n),
        16 => op_8xy6(mach, x, y, n),
        17 => op_8xy7(mach, x, y, n),
        18 => op_8xye(mach, x, y, n),
        19 => op_9xy0(mach, x, y, n),
        20 => op_annn(mach, x, y, n),
        21 => op_bnnn(mach, x, y, n),
        22 => op_cxnn(mach, x, y, n),
        23 => op_dxyn(mach, x, y, n),
        24 => op_ex9e(mach, x, y, n),
        25 => op_exa1(mach, x, y, n),
        26 => op_fx07(mach, x, y, n),
        27 => op_fx0a(mach, x, y, n),
        28 => op_fx15(mach, x, y, n),
        29 => op_fx18(mach, x, y, n),
        30 => op_fx1e(mach, x, y, n),
        31 => op_fx29(mach, x, y, n),
        32 => op_fx33(mach, x, y, n),
        33 => op_fx55(mach, x, y, n),
        34 => op_fx65(mach, x, y, n),
        _ => panic!("invalid operation index")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_00ee() {
        let mut mach = machine::create_machine();
        machine::call(&mut mach, 0x300);
        op_00ee(&mut mach, 0, 0, 0);

        assert_eq!(machine::START_USER_SPACE, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_1nnn() {
        let mut mach = machine::create_machine();
        op_1nnn(&mut mach, 0, 0, 0x300);

        assert_eq!(0x300, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_2nnn() {
        let mut mach = machine::create_machine();
        op_2nnn(&mut mach, 0, 0, 0x300);

        assert_eq!(0x300, machine::get_program_counter(&mach));
        assert_eq!(Some(machine::START_USER_SPACE), machine::peek_stack(&mut mach));
    }

    #[test]
    fn test_op_3xnn_pass() {
        let mut mach = machine::create_machine();
        machine::set_register(&mut mach, 0x0, 0x8);
        op_3xnn(&mut mach, 0x0, 0, 0x8);

        assert_eq!(machine::START_USER_SPACE + 4, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_3xnn_fail() {
        let mut mach = machine::create_machine();
        machine::set_register(&mut mach, 0x0, 0x8);
        op_3xnn(&mut mach, 0x0, 0, 0x1);

        assert_eq!(machine::START_USER_SPACE + 2, machine::get_program_counter(&mach));
    }

     #[test]
    fn test_op_4xnn_pass() {
        let mut mach = machine::create_machine();
        machine::set_register(&mut mach, 0x0, 0x8);
        op_4xnn(&mut mach, 0x0, 0, 0x1);

        assert_eq!(machine::START_USER_SPACE + 4, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_4xnn_fail() {
        let mut mach = machine::create_machine();
        machine::set_register(&mut mach, 0x0, 0x8);
        op_4xnn(&mut mach, 0x0, 0, 0x8);

        assert_eq!(machine::START_USER_SPACE + 2, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_5xy0_pass() {
        let mut mach = machine::create_machine();
        machine::set_register(&mut mach, 0x0, 0x8);
        machine::set_register(&mut mach, 0x1, 0x8);
        op_5xy0(&mut mach, 0x0, 0x1, 0);

        assert_eq!(machine::START_USER_SPACE + 4, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_5xy0_fail() {
        let mut mach = machine::create_machine();
        machine::set_register(&mut mach, 0x0, 0x8);
        machine::set_register(&mut mach, 0x1, 0x7);
        op_5xy0(&mut mach, 0x0, 0x1, 0);

        assert_eq!(machine::START_USER_SPACE + 2, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_6xnn() {
        let mut machine = machine::create_machine();
        op_6xnn(&mut machine, 0x0, 0, 0x8);

        assert_eq!(0x8, machine::get_register(&machine, 0x0));
    }

    #[test]
    fn test_op_7xnn() {
        let mut machine = machine::create_machine();
        machine::set_register(&mut machine, 0x0, 0x4);
        op_7xnn(&mut machine, 0x0, 0, 0x4);

        assert_eq!(0x8, machine::get_register(&machine, 0x0));

        op_7xnn(&mut machine, 0x0, 0, 0xfe);

        assert_eq!(0x6, machine::get_register(&machine, 0x0));
        assert_eq!(0x0, machine::get_register(&machine, 0xf));
    }

    #[test]
    fn test_op_8xy0() {
        let mut machine = machine::create_machine();
        machine::set_register(&mut machine, 0x0, 0x1);
        machine::set_register(&mut machine, 0x1, 0x8);
        op_8xy0(&mut machine, 0x0, 0x1, 0);

        assert_eq!(0x8, machine::get_register(&machine, 0x0));
    }

    #[test]
    fn test_op_8xy1() {
        let mut machine = machine::create_machine();
        machine::set_register(&mut machine, 0x0, 0x4);
        machine::set_register(&mut machine, 0x1, 0x1);
        op_8xy1(&mut machine, 0x0, 0x1, 0);

        assert_eq!(0x5, machine::get_register(&machine, 0x0));

        machine::set_register(&mut machine, 0x0, 0x1);
        op_8xy1(&mut machine, 0x0, 0x1, 0);

        assert_eq!(0x1, machine::get_register(&machine, 0x0));
    }

    #[test]
    fn test_op_8xy2() {
        let mut machine = machine::create_machine();
        machine::set_register(&mut machine, 0x0, 0x4);
        machine::set_register(&mut machine, 0x1, 0x1);
        op_8xy2(&mut machine, 0x0, 0x1, 0);

        assert_eq!(0x0, machine::get_register(&machine, 0x0));

        machine::set_register(&mut machine, 0x0, 0x5);
        op_8xy2(&mut machine, 0x0, 0x1, 0);

        assert_eq!(0x1, machine::get_register(&machine, 0x0));
    }

    #[test]
    fn test_op_8xy3() {
        let mut machine = machine::create_machine();
        machine::set_register(&mut machine, 0x0, 0x4);
        machine::set_register(&mut machine, 0x1, 0x1);
        op_8xy3(&mut machine, 0x0, 0x1, 0);

        assert_eq!(0x5, machine::get_register(&machine, 0x0));

        machine::set_register(&mut machine, 0x0, 0x5);
        op_8xy3(&mut machine, 0x0, 0x1, 0);

        assert_eq!(0x4, machine::get_register(&machine, 0x0));
    }

    #[test]
    fn test_op_8xy4() {
        let mut machine = machine::create_machine();
        machine::set_register(&mut machine, 0x0, 0x8);
        machine::set_register(&mut machine, 0x1, 0x9);
        op_8xy4(&mut machine, 0x0, 0x1, 0);

        assert_eq!(0x11, machine::get_register(&machine, 0x0));
        assert_eq!(0x0, machine::get_register(&machine, 0xf));

        machine::set_register(&mut machine, 0x0, 0xf8);
        op_8xy4(&mut machine, 0x0, 0x1, 0);

        assert_eq!(0x1, machine::get_register(&machine, 0x0));
        assert_eq!(0x1, machine::get_register(&machine, 0xf));
    }

    #[test]
    fn test_op_8xy5() {
        let mut machine = machine::create_machine();
        machine::set_register(&mut machine, 0x0, 0x9);
        machine::set_register(&mut machine, 0x1, 0x8);
        op_8xy5(&mut machine, 0x0, 0x1, 0);

        assert_eq!(0x1, machine::get_register(&machine, 0x0));
        assert_eq!(0x1, machine::get_register(&machine, 0xf));

        machine::set_register(&mut machine, 0x0, 0x1);
        op_8xy5(&mut machine, 0x0, 0x1, 0);

        assert_eq!(0xf9, machine::get_register(&machine, 0x0));
        assert_eq!(0x0, machine::get_register(&machine, 0xf));
    }

    #[test]
    fn test_op_8xy6() {
        let mut machine = machine::create_machine();
        machine::set_register(&mut machine, 0x0, 0x3);
        op_8xy6(&mut machine, 0x0, 0, 0);

        assert_eq!(0x1, machine::get_register(&machine, 0xf));
        assert_eq!(0x1, machine::get_register(&machine, 0x0));
    }

    #[test]
    fn test_op_8xy7() {
        let mut machine = machine::create_machine();
        machine::set_register(&mut machine, 0x0, 0x8);
        machine::set_register(&mut machine, 0x1, 0x9);
        op_8xy7(&mut machine, 0x0, 0x1, 0);

        assert_eq!(0x1, machine::get_register(&machine, 0x0));
        assert_eq!(0x1, machine::get_register(&machine, 0xf));

        machine::set_register(&mut machine, 0x0, 0xa);
        op_8xy7(&mut machine, 0x0, 0x1, 0);

        assert_eq!(0xff, machine::get_register(&machine, 0x0));
        assert_eq!(0x0, machine::get_register(&machine, 0xf));
    }

    #[test]
    fn test_op_8xye() {
        let mut machine = machine::create_machine();
        machine::set_register(&mut machine, 0x0, 0xff);
        op_8xye(&mut machine, 0x0, 0, 0);

        assert_eq!(0x1, machine::get_register(&machine, 0xf));
        assert_eq!(0xfe, machine::get_register(&machine, 0x0));
    }

    #[test]
    fn test_op_9xy0_pass() {
        let mut mach = machine::create_machine();
        machine::set_register(&mut mach, 0x0, 0x8);
        machine::set_register(&mut mach, 0x1, 0x7);
        op_9xy0(&mut mach, 0x0, 0x1, 0);

        assert_eq!(machine::START_USER_SPACE + 4, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_9xy0_fail() {
        let mut mach = machine::create_machine();
        machine::set_register(&mut mach, 0x0, 0x8);
        machine::set_register(&mut mach, 0x1, 0x8);
        op_9xy0(&mut mach, 0x0, 0x1, 0);

        assert_eq!(machine::START_USER_SPACE + 2, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_annn() {
        let mut mach = machine::create_machine();
        op_annn(&mut mach, 0, 0, 0x300);

        assert_eq!(0x300, machine::get_address_register(&mach));
    }

    #[test]
    fn test_op_bnnn() {
        let mut mach = machine::create_machine();
        machine::set_register(&mut mach, 0x0, 0x50);
        op_bnnn(&mut mach, 0, 0, 0x300);

        assert_eq!(0x350, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_ex9e_pass() {
        let mut mach = machine::create_machine();
        machine::set_key(&mut mach, 0, true);
        op_ex9e(&mut mach, 0x0, 0, 0);

        assert_eq!(machine::START_USER_SPACE + 4, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_ex9e_fail() {
        let mut mach = machine::create_machine();
        op_ex9e(&mut mach, 0x0, 0, 0);

        assert_eq!(machine::START_USER_SPACE + 2, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_exa1_pass() {
        let mut mach = machine::create_machine();
        op_exa1(&mut mach, 0x0, 0, 0);

        assert_eq!(machine::START_USER_SPACE + 4, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_exa1_fail() {
        let mut mach = machine::create_machine();
        machine::set_key(&mut mach, 0, true);
        op_exa1(&mut mach, 0x0, 0, 0);

        assert_eq!(machine::START_USER_SPACE + 2, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_fx07() {
        let mut mach = machine::create_machine();
        machine::set_delay_timer(&mut mach, 10);
        op_fx07(&mut mach, 0xa, 0, 0);

        assert_eq!(10, machine::get_delay_timer(&mach));
    }

    #[test]
    fn test_op_fx0a() {
        let mut mach = machine::create_machine();
        op_fx0a(&mut mach, 0x1, 0, 0);

        assert_eq!(machine::Flags::WaitingForKeypress, machine::get_flag(&mach));
        assert_eq!(1, machine::get_target_register(&mach));
    }

    #[test]
    fn test_op_fx15() {
        let mut mach = machine::create_machine();
        machine::set_register(&mut mach, 0x0, 0xa);
        op_fx15(&mut mach, 0x0, 0, 0);

        assert_eq!(10, machine::get_delay_timer(&mach));
    }

    #[test]
    fn test_op_fx18() {
        let mut mach = machine::create_machine();
        machine::set_register(&mut mach, 0x0, 0xa);
        op_fx18(&mut mach, 0x0, 0, 0);

        assert_eq!(10, machine::get_sound_timer(&mach));
    }

    #[test]
    fn test_op_fx1e() {
        let mut mach = machine::create_machine();
        machine::set_address_register(&mut mach, 0x9);
        machine::set_register(&mut mach, 0x0, 0x2);
        op_fx1e(&mut mach, 0x0, 0, 0);

        assert_eq!(0xb, machine::get_address_register(&mach));
    }

    #[test]
    fn test_op_fx29() {
        let mut mach = machine::create_machine();
        machine::set_register(&mut mach, 0x0, 0x1);
        op_fx29(&mut mach, 0x0, 0, 0);

        assert_eq!(machine::START_FONT + render::FONT_BYTES_PER_CHAR, machine::get_address_register(&mach));
    }

    #[test]
    fn test_op_fx33() {
        let mut mach = machine::create_machine();
        machine::set_address_register(&mut mach, machine::START_USER_SPACE);
        machine::set_register(&mut mach, 0x0, 0xfe);
        op_fx33(&mut mach, 0x0, 0, 0);

        assert_eq!(2, machine::read_memory(&mach, machine::START_USER_SPACE));
        assert_eq!(5, machine::read_memory(&mach, machine::START_USER_SPACE + 1));
        assert_eq!(4, machine::read_memory(&mach, machine::START_USER_SPACE + 2));
    }

    #[test]
    fn test_op_fx55() {
        let mut mach = machine::create_machine();
        machine::set_address_register(&mut mach, machine::START_USER_SPACE);
        for x in 0..machine::NUM_REGISTERS {
            machine::set_register(&mut mach, x, x as u8);
        }

        op_fx55(&mut mach, 0xf, 0, 0);

        for x in 0..machine::NUM_REGISTERS {
            assert_eq!(x as u8, machine::read_memory(&mach, machine::START_USER_SPACE + x));
        }
    }

    #[test]
    fn test_op_fx65() {
        let mut mach = machine::create_machine();
        machine::set_address_register(&mut mach, machine::START_USER_SPACE);

        for x in 0..machine::NUM_REGISTERS {
            machine::write_memory(&mut mach, machine::START_USER_SPACE + x, x as u8);
        }

        op_fx65(&mut mach, 0xf, 0, 0);

        for x in 0..machine::NUM_REGISTERS {
            assert_eq!(x as u8, machine::get_register(&mach, x));
        }
    }
}