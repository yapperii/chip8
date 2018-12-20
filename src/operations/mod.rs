use machine;

pub fn op_0NNN(mach: &mut machine::Machine) {
    // probably not needed
}

// clears the screen
pub fn op_00E0(mach: &mut machine::Machine) {
    //not implemented yet
}

// returns from a function
pub fn op_00EE(mach: &mut machine::Machine) {
    machine::ret(mach);
}

pub fn op_1NNN(mach: &mut machine::Machine, address: usize) {
    machine::jump(mach, address);
}

pub fn op_2NNN(mach: &mut machine::Machine, address: usize) {
    machine::jump(mach, address);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_00EE() {
        let mut mach = machine::create_machine();
        machine::jump(&mut mach, 0x300);
        op_00EE(&mut mach);

        assert_eq!(machine::START_USER_SPACE, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_1NNN() {
        let mut mach = machine::create_machine();
        op_1NNN(&mut mach, 0x300);

        assert_eq!(0x300, machine::get_program_counter(&mach));
        assert_eq!(machine::START_USER_SPACE, machine::pop_stack(&mut mach));
    }

    #[test]
    fn test_op_2NNN() {
        let mut mach = machine::create_machine();
        op_2NNN(&mut mach, 0x300);

        assert_eq!(0x300, machine::get_program_counter(&mach));
        assert_eq!(machine::START_USER_SPACE, machine::pop_stack(&mut mach));
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
}