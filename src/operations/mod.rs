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
        machine::set_register(&mut mach, 0, 0x8);
        op_3XNN(&mut mach, 0, 0x8);

        assert_eq!(machine::START_USER_SPACE + 2, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_3XNN_fail() {
        let mut mach = machine::create_machine();
        machine::set_register(&mut mach, 0, 0x8);
        op_3XNN(&mut mach, 0, 0x1);

        assert_eq!(machine::START_USER_SPACE, machine::get_program_counter(&mach));
    }

     #[test]
    fn test_op_4XNN_pass() {
        let mut mach = machine::create_machine();
        machine::set_register(&mut mach, 0, 0x8);
        op_4XNN(&mut mach, 0, 0x1);

        assert_eq!(machine::START_USER_SPACE + 2, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_4XNN_fail() {
        let mut mach = machine::create_machine();
        machine::set_register(&mut mach, 0, 0x8);
        op_4XNN(&mut mach, 0, 0x8);

        assert_eq!(machine::START_USER_SPACE, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_5XY0_pass() {
        let mut mach = machine::create_machine();
        machine::set_register(&mut mach, 0, 0x8);
        machine::set_register(&mut mach, 1, 0x8);
        op_5XY0(&mut mach, 0, 1);

        assert_eq!(machine::START_USER_SPACE + 2, machine::get_program_counter(&mach));
    }

    #[test]
    fn test_op_5XY0_fail() {
        let mut mach = machine::create_machine();
        machine::set_register(&mut mach, 0, 0x8);
        machine::set_register(&mut mach, 1, 0x7);
        op_5XY0(&mut mach, 0, 1);

        assert_eq!(machine::START_USER_SPACE, machine::get_program_counter(&mach));
    }
}