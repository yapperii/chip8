const MEM_SIZE: usize = 4096;
const START_USER_SPACE: usize = 0x200;
const NUM_REGISTERS: usize = 16;
const STACK_SIZE: usize = 24;

pub struct Ram {
    memory: [u8; MEM_SIZE],
}

pub struct Registers {
    general_registers: [u16; NUM_REGISTERS],
    address_register: u16,
}

pub struct Machine {
    ram: Ram,
    registers: Registers,
    stack: Vec<u16>,
}

pub fn create_machine() -> Machine {
    let ram = Ram {memory: [0; MEM_SIZE]};
    let registers = Registers {general_registers: [0; NUM_REGISTERS], address_register: 0};
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
    fn creation_registers_are_zeroes() {
        let machine = create_machine();
        for i in 0 .. NUM_REGISTERS {
            assert_eq!(0, machine.registers.general_registers[i])
        }

        assert_eq!(0, machine.registers.address_register);
    }

    #[test]
    fn creation_stack_len() {
        let machine = create_machine();
        assert_eq!(0, machine.stack.len());
    }

    #[test]
    fn push_pop() {
        let mut machine = create_machine();
        push_stack(1, &mut machine);
        assert_eq!(1, pop_stack(&mut machine));

        push_stack(1, &mut machine);
        push_stack(2, &mut machine);
        push_stack(3, &mut machine);

        assert_eq!(3, machine.stack.len());

        assert_eq!(3, pop_stack(&mut machine));
        assert_eq!(2, pop_stack(&mut machine));
        assert_eq!(1, pop_stack(&mut machine));

        assert_eq!(0, machine.stack.len());
    }

    #[test]
    #[should_panic]
    fn stack_overflow() {
        let mut machine = create_machine();
        for _i in 0 .. (STACK_SIZE + 1) {
            push_stack(0, &mut machine);
        }
    }

    #[test]
    #[should_panic]
    fn stack_underflow() {
        let mut machine = create_machine();
        pop_stack(&mut machine);
    }
}