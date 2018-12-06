const MEM_SIZE: usize = 4096;
const USER_SPACE_MIN: usize = 0x200;
const NUM_REGISTERS: usize = 16;
const STACK_SIZE: usize = 24;

pub struct Ram {
    memory: [u8; MEM_SIZE],
}

pub struct Registers {
    registers: [u16; NUM_REGISTERS],
}

pub struct Machine {
    ram: Ram,
    registers: Registers,
    stack: Vec<u16>,
}

pub fn create_machine() -> Machine {
    let ram = Ram {memory: [0; MEM_SIZE]};
    let registers = Registers {registers: [0; NUM_REGISTERS]};
    Machine {ram: ram, registers: registers, stack: Vec::with_capacity(STACK_SIZE)}
}

pub fn push_stack(val: u16, machine: &mut Machine) {
    if machine.stack.len() >= STACK_SIZE {
        panic!("stack overrun!")
    }

    machine.stack.push(val);
}

pub fn pop_stack(machine: &mut Machine) -> u16 {
    let popped = machine.stack.pop();
    match popped {
        Some(x) => x,
        None => panic!("stack underrun!")
    }
}    


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation_ram_is_zeroes() {
        let machine = create_machine();
        for i in 0 .. MEM_SIZE {
            assert_eq!(0, machine.ram.memory[i])
        }
    }

    #[test]
    fn creation_stack_len() {
        let machine = create_machine();
        assert_eq!(0, machine.stack.len());
    }
}