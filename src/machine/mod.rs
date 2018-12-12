
const MEM_SIZE: usize = 4096;
const START_USER_SPACE: usize = 0x200;
const NUM_REGISTERS: usize = 16;
const STACK_SIZE: usize = 24;

pub struct Ram {
    memory: [u8; MEM_SIZE],
}

pub struct Registers {
    general_registers: [u8; NUM_REGISTERS],
    address_register: u16,
}

pub struct Machine {
    ram: Ram,
    registers: Registers,
    stack: Vec<usize>,
    program_counter: usize,
}

pub fn create_machine() -> Machine {
    let ram = Ram {memory: [0; MEM_SIZE]};
    let registers = Registers {general_registers: [0; NUM_REGISTERS], address_register: 0};
    Machine {ram: ram, registers: registers, stack: Vec::with_capacity(STACK_SIZE), program_counter: 0}
}

pub fn push_stack(machine: &mut Machine, val: usize) {
    if machine.stack.len() >= STACK_SIZE {
        panic!("stack overrun!")
    }

    machine.stack.push(val);
}

pub fn pop_stack(machine: &mut Machine) -> usize {
    let popped = machine.stack.pop();
    match popped {
        Some(x) => x,
        None => panic!("stack underrun!")
    }
}

fn get_program_counter(machine: &Machine) -> usize {
    machine.program_counter
}

fn set_program_counter(machine: &mut Machine, counter: usize) {
    if counter >= START_USER_SPACE  && counter < MEM_SIZE {
        machine.program_counter = counter;
    }else
    {
        panic!("program counter out of range");
    }
}

pub fn increment_program_counter(machine: &mut Machine) {
    machine.program_counter += 2;
}

pub fn jump(machine: &mut Machine, address: usize) {
    let pc = get_program_counter(machine);
    push_stack(machine, pc);
    set_program_counter(machine, address);
}

pub fn ret(machine: &mut Machine) {
    let pc = pop_stack(machine);
    set_program_counter(machine, pc);
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
    fn creation_program_counter_is_zero() {
        let machine = create_machine();
        assert_eq!(0, machine.program_counter);
    }

    #[test]
    fn push_pop() {
        let mut machine = create_machine();
        push_stack(&mut machine, 1);
        assert_eq!(1, pop_stack(&mut machine));

        push_stack(&mut machine,1 );
        push_stack(&mut machine, 2);
        push_stack(&mut machine, 3);

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
            push_stack(&mut machine, 0);
        }
    }

    #[test]
    #[should_panic]
    fn stack_underflow() {
        let mut machine = create_machine();
        pop_stack(&mut machine);
    }

    #[test]
    fn change_program_counter() {
        let mut machine = create_machine();
        let pc: usize = START_USER_SPACE;
        set_program_counter(&mut machine, pc);
        assert_eq!(START_USER_SPACE, get_program_counter(&machine));
    }

    #[test]
    #[should_panic]
    fn change_program_counter_before_user_space() {
        let mut machine = create_machine();
        let pc: usize = START_USER_SPACE -1;
        set_program_counter(&mut machine, pc);
        assert_eq!(START_USER_SPACE, get_program_counter(&machine));
    }

    #[test]
    #[should_panic]
    fn change_program_counter_too_high() {
        let mut machine = create_machine();
        let pc: usize = MEM_SIZE;
        set_program_counter(&mut machine, pc);
        assert_eq!(START_USER_SPACE, get_program_counter(&machine));
    }
}
