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
    machine::jump(mach, address)
}

pub fn op_2NNN(mach: &mut machine::Machine, address: usize) {
    machine::jump(mach, address)
}

pub fn op_3XNN(mach: &mut machine::Machine, val: u16) {
    //get register value
    //compare val to register
}