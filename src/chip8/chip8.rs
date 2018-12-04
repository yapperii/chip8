mod chip8 {
    struct Ram {
        const MEM_SIZE: u32 = 4096;
        const USER_SPACE_MIN = 0x200;
        memory: u8[MEM_SIZE];
    }

    struct Registers {
        const NUM_REGISTERS: u32 = 16;
        registers: u16[NUM_REGISTERS];
    }

    struct Machine {
        ram: Ram;
        registers: Registers;
        const STACK_SIZE: u32 = 24;
        stack: vector<u16>;
    }

    pub fn power_on(&mut machine: Machine) {
        machine.stack = Vec<u16>::with_capacity(machine.STACK_SIZE);
    }

    pub fn push_stacK(val: u16, &mut machine: Machine) {
        if machine.stack.len() >= machine.STACK_SIZE {
            panic!("stack overrun!")
        }

        machine.stack.push(val);
    }

    pub fn pop_stack(&mut machine: Machine) -> u16 {
        if machine.stack.len() <= 0 {
            panic!("stack underrun!")
        }

        machine.stack.pop()
    }    
}