use machine;
use screen_buffer;

pub fn op_0nnn(mach: &mut machine::Machine, _x: u16, _y: u16, _n: u16) {
    // probably not needed
    mach.increment_program_counter();
}

pub fn op_00e0(mach: &mut machine::Machine, _x: u16, _y: u16, _n: u16) {
    mach.get_screenbuffer().clear_screen();
    mach.increment_program_counter();
}

pub fn op_00ee(mach: &mut machine::Machine, _x: u16, _y: u16, _n: u16) {
    mach.ret();
}

pub fn op_1nnn(mach: &mut machine::Machine, _x: u16, _y: u16, n: u16) {
    mach.jump(n as usize);
}

pub fn op_2nnn(mach: &mut machine::Machine, _x: u16, _y: u16, n: u16) {
    mach.call(n as usize);
}

pub fn op_3xnn(mach: &mut machine::Machine, x: u16, _y: u16, n: u16) {
    let vx = mach.get_register(x as usize) as u16;
    if vx == n {
        mach.increment_program_counter();
    }
    mach.increment_program_counter();
}

pub fn op_4xnn(mach: &mut machine::Machine, x: u16, _y: u16, n: u16) {
    let vx = mach.get_register(x as usize) as u16;
    if vx != n {
        mach.increment_program_counter();
    }
    mach.increment_program_counter();
}

pub fn op_5xy0(mach: &mut machine::Machine, x: u16, y: u16, _n: u16) {
    let vx = mach.get_register(x as usize);
    let vy = mach.get_register(y as usize);
    if vx == vy {
        mach.increment_program_counter();
    }
    mach.increment_program_counter();
}

pub fn op_6xnn(mach: &mut machine::Machine, x: u16, _y: u16, n: u16) {
    mach.set_register(x as usize, n as u8);
    mach.increment_program_counter();
}

pub fn op_7xnn(mach: &mut machine::Machine, x: u16, _y: u16, n: u16) {
    let vx = mach.get_register(x as usize);
    let sum: u16 = vx as u16 + n as u16;
    mach.set_register(x as usize, sum as u8);
    mach.increment_program_counter();
}

pub fn op_8xy0(mach: &mut machine::Machine, x: u16, y: u16, _n: u16) {
    let vy = mach.get_register(y as usize);
    mach.set_register(x as usize, vy);
    mach.increment_program_counter();
}

pub fn op_8xy1(mach: &mut machine::Machine, x: u16, y: u16, _n: u16) {
    let vx = mach.get_register(x as usize);
    let vy = mach.get_register(y as usize);
    mach.set_register(x as usize, vx | vy);
    mach.increment_program_counter();
}

pub fn op_8xy2(mach: &mut machine::Machine, x: u16, y: u16, _n: u16) {
    let vx = mach.get_register(x as usize);
    let vy = mach.get_register(y as usize);
    mach.set_register(x as usize, vx & vy);
    mach.increment_program_counter();
}

pub fn op_8xy3(mach: &mut machine::Machine, x: u16, y: u16, _n: u16) {
    let vx = mach.get_register(x as usize);
    let vy = mach.get_register(y as usize);
    mach.set_register(x as usize, vx ^ vy);
    mach.increment_program_counter();
}

pub fn op_8xy4(mach: &mut machine::Machine, x: u16, y: u16, _n: u16) {
    let vx = mach.get_register(x as usize);
    let vy = mach.get_register(y as usize);
    let sum: u16 = vx as u16 + vy as u16;
    mach.set_register(x as usize, sum as u8);
    mach.set_register(0xf, if sum > 0xff { 1 } else { 0 });
    mach.increment_program_counter();
}

pub fn op_8xy5(mach: &mut machine::Machine, x: u16, y: u16, _n: u16) {
    let vx = mach.get_register(x as usize);
    let vy = mach.get_register(y as usize);
    let difference: i16 = vx as i16 - vy as i16;
    mach.set_register(x as usize, (0x100 + difference) as u8);
    mach.set_register(0xf, if difference < 0 { 0 } else { 1 });
    mach.increment_program_counter();
}

pub fn op_8xy6(mach: &mut machine::Machine, x: u16, _y: u16, _n: u16) {
    let vx = mach.get_register(x as usize);
    mach.set_register(0xf, vx & 0x1);
    mach.set_register(x as usize, vx >> 1);
    mach.increment_program_counter();
}

pub fn op_8xy7(mach: &mut machine::Machine, x: u16, y: u16, _n: u16) {
    let vx = mach.get_register(x as usize);
    let vy = mach.get_register(y as usize);
    let difference: i16 = vy as i16 - vx as i16;
    mach.set_register(x as usize, (0x100 + difference) as u8);
    mach.set_register(0xf, if vy > vx { 1 } else { 0 });
    mach.increment_program_counter();
}

pub fn op_8xye(mach: &mut machine::Machine, x: u16, _y: u16, _n: u16) {
    let vx = mach.get_register(x as usize);
    mach.set_register(0xf, if (vx & 0x80) > 0 { 1 } else { 0 });
    mach.set_register(x as usize, vx << 1);
    mach.increment_program_counter();
}

pub fn op_9xy0(mach: &mut machine::Machine, x: u16, y: u16, _n: u16) {
    let vx = mach.get_register(x as usize) as u16;
    let vy = mach.get_register(y as usize) as u16;
    if vx != vy {
        mach.increment_program_counter();
    }
    mach.increment_program_counter();
}

pub fn op_annn(mach: &mut machine::Machine, _x: u16, _y: u16, n: u16) {
    mach.set_address_register(n as usize);
    mach.increment_program_counter();
}

pub fn op_bnnn(mach: &mut machine::Machine, _x: u16, _y: u16, n: u16) {
    let v0 = mach.get_register(0x0);
    let address = n as usize + v0 as usize;
    mach.jump(address);
}

pub fn op_cxnn(mach: &mut machine::Machine, x: u16, _y: u16, n: u16) {
    let r = rand::random::<u8>();
    mach.set_register(x as usize, n as u8 & r);
    mach.increment_program_counter();
}

pub fn op_dxyn(mach: &mut machine::Machine, x: u16, y: u16, n: u16) {
    let vx = mach.get_register(x as usize);
    let vy = mach.get_register(y as usize);
    let base_address = mach.get_address_register();
    let mut flipped = false;
    for i in 0..n {
        let mem_val = mach.read_memory(base_address + i as usize);
        let mut row: [bool; 8] = [false; 8];
        for j in 0..8 {
            row[j] = (mem_val & (1 << (8 - j - 1))) != 0;
        }

        flipped |= mach
            .get_screenbuffer()
            .blit_texture_row(vx, vy + i as u8, &row);
    }

    mach.set_register(0xf, if flipped { 1 } else { 0 });
    mach.increment_program_counter();
}

pub fn op_ex9e(mach: &mut machine::Machine, x: u16, _y: u16, _n: u16) {
    let vx = mach.get_register(x as usize);
    if mach.get_key(vx as usize) {
        mach.increment_program_counter();
    }
    mach.increment_program_counter();
}

pub fn op_exa1(mach: &mut machine::Machine, x: u16, _y: u16, _n: u16) {
    let vx = mach.get_register(x as usize);
    if !mach.get_key(vx as usize) {
        mach.increment_program_counter();
    }
    mach.increment_program_counter();
}

pub fn op_fx07(mach: &mut machine::Machine, x: u16, _y: u16, _n: u16) {
    let delay = mach.get_delay_timer();
    mach.set_register(x as usize, delay);
    mach.increment_program_counter();
}

pub fn op_fx0a(mach: &mut machine::Machine, x: u16, _y: u16, _n: u16) {
    mach.set_flag(machine::Flags::WaitingForKeypress);
    mach.set_target_register(x as usize);
    mach.increment_program_counter();
}

pub fn op_fx15(mach: &mut machine::Machine, x: u16, _y: u16, _n: u16) {
    let vx = mach.get_register(x as usize);
    mach.set_delay_timer(vx);
    mach.increment_program_counter();
}

pub fn op_fx18(mach: &mut machine::Machine, x: u16, _y: u16, _n: u16) {
    let vx = mach.get_register(x as usize);
    mach.set_sound_timer(vx);
    mach.increment_program_counter();
}

pub fn op_fx1e(mach: &mut machine::Machine, x: u16, _y: u16, _n: u16) {
    let address_register = mach.get_address_register();
    let vx = mach.get_register(x as usize);
    mach.set_address_register(address_register + vx as usize);
    mach.increment_program_counter();
}

pub fn op_fx29(mach: &mut machine::Machine, x: u16, _y: u16, _n: u16) {
    let vx = mach.get_register(x as usize);
    let address = machine::START_FONT + (vx as usize * screen_buffer::FONT_BYTES_PER_CHAR);
    mach.set_address_register(address);
    mach.increment_program_counter();
}

pub fn op_fx33(mach: &mut machine::Machine, x: u16, _y: u16, _n: u16) {
    let vx = mach.get_register(x as usize);
    let base_address = mach.get_address_register();
    let ones = vx % 10;
    let tens = (vx / 10) % 10;
    let hundreds = (vx / 100) % 10;
    mach.write_memory(base_address, hundreds);
    mach.write_memory(base_address + 1, tens);
    mach.write_memory(base_address + 2, ones);
    mach.increment_program_counter();
}

pub fn op_fx55(mach: &mut machine::Machine, x: u16, _y: u16, _n: u16) {
    let base_address = mach.get_address_register();
    for i in 0..((x + 1) as usize) {
        let vi = mach.get_register(i);
        mach.write_memory(base_address + i, vi);
    }
    mach.increment_program_counter();
}

pub fn op_fx65(mach: &mut machine::Machine, x: u16, _y: u16, _n: u16) {
    let base_address = mach.get_address_register();
    for i in 0..((x + 1) as usize) {
        let mem = mach.read_memory(base_address + i);
        mach.set_register(i, mem);
    }
    mach.increment_program_counter();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_00ee() {
        let mut mach = machine::Machine::new();
        mach.call(0x300);
        op_00ee(&mut mach, 0, 0, 0);

        assert_eq!(machine::START_USER_SPACE + 2, mach.get_program_counter());
    }

    #[test]
    fn test_op_1nnn() {
        let mut mach = machine::Machine::new();
        op_1nnn(&mut mach, 0, 0, 0x300);

        assert_eq!(0x300, mach.get_program_counter());
    }

    #[test]
    fn test_op_2nnn() {
        let mut mach = machine::Machine::new();
        op_2nnn(&mut mach, 0, 0, 0x300);

        assert_eq!(0x300, mach.get_program_counter());
        assert_eq!(Some(machine::START_USER_SPACE + 2), mach.peek_stack());
    }

    #[test]
    fn test_op_3xnn_pass() {
        let mut mach = machine::Machine::new();
        mach.set_register(0x0, 0x8);
        op_3xnn(&mut mach, 0x0, 0, 0x8);

        assert_eq!(machine::START_USER_SPACE + 4, mach.get_program_counter());
    }

    #[test]
    fn test_op_3xnn_fail() {
        let mut mach = machine::Machine::new();
        mach.set_register(0x0, 0x8);
        op_3xnn(&mut mach, 0x0, 0, 0x1);

        assert_eq!(machine::START_USER_SPACE + 2, mach.get_program_counter());
    }

    #[test]
    fn test_op_4xnn_pass() {
        let mut mach = machine::Machine::new();
        mach.set_register(0x0, 0x8);
        op_4xnn(&mut mach, 0x0, 0, 0x1);

        assert_eq!(machine::START_USER_SPACE + 4, mach.get_program_counter());
    }

    #[test]
    fn test_op_4xnn_fail() {
        let mut mach = machine::Machine::new();
        mach.set_register(0x0, 0x8);
        op_4xnn(&mut mach, 0x0, 0, 0x8);

        assert_eq!(machine::START_USER_SPACE + 2, mach.get_program_counter());
    }

    #[test]
    fn test_op_5xy0_pass() {
        let mut mach = machine::Machine::new();
        mach.set_register(0x0, 0x8);
        mach.set_register(0x1, 0x8);
        op_5xy0(&mut mach, 0x0, 0x1, 0);

        assert_eq!(machine::START_USER_SPACE + 4, mach.get_program_counter());
    }

    #[test]
    fn test_op_5xy0_fail() {
        let mut mach = machine::Machine::new();
        mach.set_register(0x0, 0x8);
        mach.set_register(0x1, 0x7);
        op_5xy0(&mut mach, 0x0, 0x1, 0);

        assert_eq!(machine::START_USER_SPACE + 2, mach.get_program_counter());
    }

    #[test]
    fn test_op_6xnn() {
        let mut machine = machine::Machine::new();
        op_6xnn(&mut machine, 0x0, 0, 0x8);

        assert_eq!(0x8, machine.get_register(0x0));
    }

    #[test]
    fn test_op_7xnn() {
        let mut machine = machine::Machine::new();
        machine.set_register(0x0, 0x4);
        op_7xnn(&mut machine, 0x0, 0, 0x4);

        assert_eq!(0x8, machine.get_register(0x0));

        op_7xnn(&mut machine, 0x0, 0, 0xfe);

        assert_eq!(0x6, machine.get_register(0x0));
        assert_eq!(0x0, machine.get_register(0xf));
    }

    #[test]
    fn test_op_8xy0() {
        let mut machine = machine::Machine::new();
        machine.set_register(0x0, 0x1);
        machine.set_register(0x1, 0x8);
        op_8xy0(&mut machine, 0x0, 0x1, 0);

        assert_eq!(0x8, machine.get_register(0x0));
    }

    #[test]
    fn test_op_8xy1() {
        let mut machine = machine::Machine::new();
        machine.set_register(0x0, 0x4);
        machine.set_register(0x1, 0x1);
        op_8xy1(&mut machine, 0x0, 0x1, 0);

        assert_eq!(0x5, machine.get_register(0x0));

        machine.set_register(0x0, 0x1);
        op_8xy1(&mut machine, 0x0, 0x1, 0);

        assert_eq!(0x1, machine.get_register(0x0));
    }

    #[test]
    fn test_op_8xy2() {
        let mut machine = machine::Machine::new();
        machine.set_register(0x0, 0x4);
        machine.set_register(0x1, 0x1);
        op_8xy2(&mut machine, 0x0, 0x1, 0);

        assert_eq!(0x0, machine.get_register(0x0));

        machine.set_register(0x0, 0x5);
        op_8xy2(&mut machine, 0x0, 0x1, 0);

        assert_eq!(0x1, machine.get_register(0x0));
    }

    #[test]
    fn test_op_8xy3() {
        let mut machine = machine::Machine::new();
        machine.set_register(0x0, 0x4);
        machine.set_register(0x1, 0x1);
        op_8xy3(&mut machine, 0x0, 0x1, 0);

        assert_eq!(0x5, machine.get_register(0x0));

        machine.set_register(0x0, 0x5);
        op_8xy3(&mut machine, 0x0, 0x1, 0);

        assert_eq!(0x4, machine.get_register(0x0));
    }

    #[test]
    fn test_op_8xy4() {
        let mut machine = machine::Machine::new();
        machine.set_register(0x0, 0x8);
        machine.set_register(0x1, 0x9);
        op_8xy4(&mut machine, 0x0, 0x1, 0);

        assert_eq!(0x11, machine.get_register(0x0));
        assert_eq!(0x0, machine.get_register(0xf));

        machine.set_register(0x0, 0xf8);
        op_8xy4(&mut machine, 0x0, 0x1, 0);

        assert_eq!(0x1, machine.get_register(0x0));
        assert_eq!(0x1, machine.get_register(0xf));
    }

    #[test]
    fn test_op_8xy5() {
        let mut machine = machine::Machine::new();
        machine.set_register(0x0, 0x9);
        machine.set_register(0x1, 0x8);
        op_8xy5(&mut machine, 0x0, 0x1, 0);

        assert_eq!(0x1, machine.get_register(0x0));
        assert_eq!(0x1, machine.get_register(0xf));

        machine.set_register(0x0, 0x1);
        op_8xy5(&mut machine, 0x0, 0x1, 0);

        assert_eq!(0xf9, machine.get_register(0x0));
        assert_eq!(0x0, machine.get_register(0xf));
    }

    #[test]
    fn test_op_8xy6() {
        let mut machine = machine::Machine::new();
        machine.set_register(0x0, 0x3);
        op_8xy6(&mut machine, 0x0, 0, 0);

        assert_eq!(0x1, machine.get_register(0xf));
        assert_eq!(0x1, machine.get_register(0x0));
    }

    #[test]
    fn test_op_8xy7() {
        let mut machine = machine::Machine::new();
        machine.set_register(0x0, 0x8);
        machine.set_register(0x1, 0x9);
        op_8xy7(&mut machine, 0x0, 0x1, 0);

        assert_eq!(0x1, machine.get_register(0x0));
        assert_eq!(0x1, machine.get_register(0xf));

        machine.set_register(0x0, 0xa);
        op_8xy7(&mut machine, 0x0, 0x1, 0);

        assert_eq!(0xff, machine.get_register(0x0));
        assert_eq!(0x0, machine.get_register(0xf));
    }

    #[test]
    fn test_op_8xye() {
        let mut machine = machine::Machine::new();
        machine.set_register(0x0, 0xff);
        op_8xye(&mut machine, 0x0, 0, 0);

        assert_eq!(0x1, machine.get_register(0xf));
        assert_eq!(0xfe, machine.get_register(0x0));
    }

    #[test]
    fn test_op_9xy0_pass() {
        let mut mach = machine::Machine::new();
        mach.set_register(0x0, 0x8);
        mach.set_register(0x1, 0x7);
        op_9xy0(&mut mach, 0x0, 0x1, 0);

        assert_eq!(machine::START_USER_SPACE + 4, mach.get_program_counter());
    }

    #[test]
    fn test_op_9xy0_fail() {
        let mut mach = machine::Machine::new();
        mach.set_register(0x0, 0x8);
        mach.set_register(0x1, 0x8);
        op_9xy0(&mut mach, 0x0, 0x1, 0);

        assert_eq!(machine::START_USER_SPACE + 2, mach.get_program_counter());
    }

    #[test]
    fn test_op_annn() {
        let mut mach = machine::Machine::new();
        op_annn(&mut mach, 0, 0, 0x300);

        assert_eq!(0x300, mach.get_address_register());
    }

    #[test]
    fn test_op_bnnn() {
        let mut mach = machine::Machine::new();
        mach.set_register(0x0, 0x50);
        op_bnnn(&mut mach, 0, 0, 0x300);

        assert_eq!(0x350, mach.get_program_counter());
    }

    #[test]
    fn test_op_ex9e_pass() {
        let mut mach = machine::Machine::new();
        mach.set_key(0, true);
        op_ex9e(&mut mach, 0x0, 0, 0);

        assert_eq!(machine::START_USER_SPACE + 4, mach.get_program_counter());
    }

    #[test]
    fn test_op_ex9e_fail() {
        let mut mach = machine::Machine::new();
        op_ex9e(&mut mach, 0x0, 0, 0);

        assert_eq!(machine::START_USER_SPACE + 2, mach.get_program_counter());
    }

    #[test]
    fn test_op_exa1_pass() {
        let mut mach = machine::Machine::new();
        op_exa1(&mut mach, 0x0, 0, 0);

        assert_eq!(machine::START_USER_SPACE + 4, mach.get_program_counter());
    }

    #[test]
    fn test_op_exa1_fail() {
        let mut mach = machine::Machine::new();
        mach.set_key(0, true);
        op_exa1(&mut mach, 0x0, 0, 0);

        assert_eq!(machine::START_USER_SPACE + 2, mach.get_program_counter());
    }

    #[test]
    fn test_op_fx07() {
        let mut mach = machine::Machine::new();
        mach.set_delay_timer(10);
        op_fx07(&mut mach, 0xa, 0, 0);

        assert_eq!(10, mach.get_delay_timer());
    }

    #[test]
    fn test_op_fx0a() {
        let mut mach = machine::Machine::new();
        op_fx0a(&mut mach, 0x1, 0, 0);

        assert_eq!(machine::Flags::WaitingForKeypress, mach.get_flag());
        assert_eq!(1, mach.get_target_register());
    }

    #[test]
    fn test_op_fx15() {
        let mut mach = machine::Machine::new();
        mach.set_register(0x0, 0xa);
        op_fx15(&mut mach, 0x0, 0, 0);

        assert_eq!(10, mach.get_delay_timer());
    }

    #[test]
    fn test_op_fx18() {
        let mut mach = machine::Machine::new();
        mach.set_register(0x0, 0xa);
        op_fx18(&mut mach, 0x0, 0, 0);

        assert_eq!(10, mach.get_sound_timer());
    }

    #[test]
    fn test_op_fx1e() {
        let mut mach = machine::Machine::new();
        mach.set_address_register(0x9);
        mach.set_register(0x0, 0x2);
        op_fx1e(&mut mach, 0x0, 0, 0);

        assert_eq!(0xb, mach.get_address_register());
    }

    #[test]
    fn test_op_fx29() {
        let mut mach = machine::Machine::new();
        mach.set_register(0x0, 0x1);
        op_fx29(&mut mach, 0x0, 0, 0);

        assert_eq!(
            machine::START_FONT + screen_buffer::FONT_BYTES_PER_CHAR,
            mach.get_address_register()
        );
    }

    #[test]
    fn test_op_fx33() {
        let mut mach = machine::Machine::new();
        mach.set_address_register(machine::START_USER_SPACE);
        mach.set_register(0x0, 0xfe);
        op_fx33(&mut mach, 0x0, 0, 0);

        assert_eq!(2, mach.read_memory(machine::START_USER_SPACE));
        assert_eq!(5, mach.read_memory(machine::START_USER_SPACE + 1));
        assert_eq!(4, mach.read_memory(machine::START_USER_SPACE + 2));
    }

    #[test]
    fn test_op_fx55() {
        let mut mach = machine::Machine::new();
        mach.set_address_register(machine::START_USER_SPACE);
        for x in 0..machine::NUM_REGISTERS {
            mach.set_register(x, x as u8);
        }

        op_fx55(&mut mach, 0xf, 0, 0);

        for x in 0..machine::NUM_REGISTERS {
            assert_eq!(x as u8, mach.read_memory(machine::START_USER_SPACE + x));
        }
    }

    #[test]
    fn test_op_fx65() {
        let mut mach = machine::Machine::new();
        mach.set_address_register(machine::START_USER_SPACE);

        for x in 0..machine::NUM_REGISTERS {
            mach.write_memory(machine::START_USER_SPACE + x, x as u8);
        }

        op_fx65(&mut mach, 0xf, 0, 0);

        for x in 0..machine::NUM_REGISTERS {
            assert_eq!(x as u8, mach.get_register(x));
        }
    }
}
