use render;

pub const MEM_SIZE: usize = 4096;
pub const START_FONT: usize = 0x50;
pub const START_USER_SPACE: usize = 0x200;
pub const NUM_REGISTERS: usize = 16;
const STACK_SIZE: usize = 24;
const NUM_KEYS: usize = 16;

pub struct Ram {
    memory: [u8; MEM_SIZE],
}

pub struct Registers {
    general_registers: [u8; NUM_REGISTERS],
    address_register: usize,
}

pub struct Timers {
    delay_timer: u8,
    sound_timer: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Flags {
    Running,
    WaitingForKeypress,
}

pub struct Machine {
    ram: Ram,
    registers: Registers,
    stack: Vec<usize>,
    program_counter: usize,
    keys: [bool; NUM_KEYS],
    timers: Timers,
    flag: Flags,
    target_register: usize,
    screen_buffer: render::ScreenBuffer,
}

fn init_font(mach: &mut Machine) {
    // zero
    write_protected_space(mach, START_FONT + 0x0, 0xf0);
    write_protected_space(mach, START_FONT + 0x1, 0x90);
    write_protected_space(mach, START_FONT + 0x2, 0x90);
    write_protected_space(mach, START_FONT + 0x3, 0x90);
    write_protected_space(mach, START_FONT + 0x4, 0xf0);

    // one
    write_protected_space(mach, START_FONT + 0x5, 0x20);
    write_protected_space(mach, START_FONT + 0x6, 0x60);
    write_protected_space(mach, START_FONT + 0x7, 0x20);
    write_protected_space(mach, START_FONT + 0x8, 0x20);
    write_protected_space(mach, START_FONT + 0x9, 0x70);

    // two
    write_protected_space(mach, START_FONT + 0xa, 0xf0);
    write_protected_space(mach, START_FONT + 0xb, 0x10);
    write_protected_space(mach, START_FONT + 0xc, 0xf0);
    write_protected_space(mach, START_FONT + 0xd, 0x80);
    write_protected_space(mach, START_FONT + 0xe, 0xf0);

    // three
    write_protected_space(mach, START_FONT + 0xf, 0xf0);
    write_protected_space(mach, START_FONT + 0x10, 0x10);
    write_protected_space(mach, START_FONT + 0x11, 0xf0);
    write_protected_space(mach, START_FONT + 0x12, 0x10);
    write_protected_space(mach, START_FONT + 0x13, 0xf0);

    // four
    write_protected_space(mach, START_FONT + 0x14, 0x90);
    write_protected_space(mach, START_FONT + 0x15, 0x90);
    write_protected_space(mach, START_FONT + 0x16, 0xf0);
    write_protected_space(mach, START_FONT + 0x17, 0x10);
    write_protected_space(mach, START_FONT + 0x18, 0x10);

    // five
    write_protected_space(mach, START_FONT + 0x19, 0xf0);
    write_protected_space(mach, START_FONT + 0x1a, 0x80);
    write_protected_space(mach, START_FONT + 0x1b, 0xf0);
    write_protected_space(mach, START_FONT + 0x1c, 0x10);
    write_protected_space(mach, START_FONT + 0x1d, 0xf0);

    // six
    write_protected_space(mach, START_FONT + 0x1e, 0xf0);
    write_protected_space(mach, START_FONT + 0x1f, 0x80);
    write_protected_space(mach, START_FONT + 0x20, 0xf0);
    write_protected_space(mach, START_FONT + 0x21, 0x90);
    write_protected_space(mach, START_FONT + 0x22, 0xf0);

    // seven
    write_protected_space(mach, START_FONT + 0x23, 0xf0);
    write_protected_space(mach, START_FONT + 0x24, 0x10);
    write_protected_space(mach, START_FONT + 0x25, 0x20);
    write_protected_space(mach, START_FONT + 0x26, 0x40);
    write_protected_space(mach, START_FONT + 0x27, 0x40);

    // eight
    write_protected_space(mach, START_FONT + 0x28, 0xf0);
    write_protected_space(mach, START_FONT + 0x29, 0x90);
    write_protected_space(mach, START_FONT + 0x2a, 0xf0);
    write_protected_space(mach, START_FONT + 0x2b, 0x90);
    write_protected_space(mach, START_FONT + 0x2c, 0xf0);

    // 9
    write_protected_space(mach, START_FONT + 0x2d, 0xf0);
    write_protected_space(mach, START_FONT + 0x2e, 0x90);
    write_protected_space(mach, START_FONT + 0x2f, 0xf0);
    write_protected_space(mach, START_FONT + 0x30, 0x10);
    write_protected_space(mach, START_FONT + 0x31, 0xf0);

    // a
    write_protected_space(mach, START_FONT + 0x32, 0xf0);
    write_protected_space(mach, START_FONT + 0x33, 0x90);
    write_protected_space(mach, START_FONT + 0x34, 0xf0);
    write_protected_space(mach, START_FONT + 0x35, 0x90);
    write_protected_space(mach, START_FONT + 0x36, 0x90);

    // b
    write_protected_space(mach, START_FONT + 0x37, 0xe0);
    write_protected_space(mach, START_FONT + 0x38, 0x90);
    write_protected_space(mach, START_FONT + 0x39, 0xe0);
    write_protected_space(mach, START_FONT + 0x3a, 0x90);
    write_protected_space(mach, START_FONT + 0x3b, 0xe0);

    // c
    write_protected_space(mach, START_FONT + 0x3c, 0xf0);
    write_protected_space(mach, START_FONT + 0x3d, 0x80);
    write_protected_space(mach, START_FONT + 0x3e, 0x80);
    write_protected_space(mach, START_FONT + 0x3f, 0x80);
    write_protected_space(mach, START_FONT + 0x40, 0xf0);

    // d
    write_protected_space(mach, START_FONT + 0x41, 0xe0);
    write_protected_space(mach, START_FONT + 0x42, 0x90);
    write_protected_space(mach, START_FONT + 0x43, 0x90);
    write_protected_space(mach, START_FONT + 0x44, 0x90);
    write_protected_space(mach, START_FONT + 0x45, 0xe0);

    // e
    write_protected_space(mach, START_FONT + 0x46, 0xf0);
    write_protected_space(mach, START_FONT + 0x47, 0x80);
    write_protected_space(mach, START_FONT + 0x48, 0xf0);
    write_protected_space(mach, START_FONT + 0x49, 0x80);
    write_protected_space(mach, START_FONT + 0x4a, 0xf0);

    // f
    write_protected_space(mach, START_FONT + 0x4b, 0xf0);
    write_protected_space(mach, START_FONT + 0x4c, 0x80);
    write_protected_space(mach, START_FONT + 0x4d, 0xf0);
    write_protected_space(mach, START_FONT + 0x4e, 0x80);
    write_protected_space(mach, START_FONT + 0x4f, 0x80);
}

pub fn create_machine() -> Machine {
    let ram = Ram {memory: [0; MEM_SIZE]};
    let registers = Registers {general_registers: [0; NUM_REGISTERS], address_register: 0};
    let mut mach = Machine{ram: ram,
            registers: registers,
            stack: Vec::with_capacity(STACK_SIZE),
            program_counter: START_USER_SPACE,
            keys: [false; NUM_KEYS],
            timers: Timers{delay_timer: 0, sound_timer: 0},
            flag: Flags::Running,
            target_register: NUM_REGISTERS,
            screen_buffer: render::create_screen_buffer()};

    init_font(&mut mach);
    return mach;
}

fn push_stack(machine: &mut Machine, val: usize) {
    if machine.stack.len() >= STACK_SIZE {
        panic!("stack overrun!")
    }

    machine.stack.push(val);
}

fn pop_stack(machine: &mut Machine) -> usize {
    let popped = machine.stack.pop();
    match popped {
        Some(x) => x,
        None => panic!("stack underrun!")
    }
}

pub fn peek_stack(machine: &mut Machine) -> Option<usize> {
    match machine.stack.len() {
        0 => None,
        n => Some(machine.stack[n-1])
    }
}

pub fn get_program_counter(machine: &Machine) -> usize {
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
    set_program_counter(machine, address);
}

pub fn call(machine: &mut Machine, address: usize) {
    increment_program_counter(machine);
    let pc = get_program_counter(machine);
    push_stack(machine, pc);
    set_program_counter(machine, address);
}

pub fn ret(machine: &mut Machine) {
    let pc = pop_stack(machine);
    set_program_counter(machine, pc);
}

pub fn get_address_register(machine: &Machine) -> usize {
    machine.registers.address_register
}

pub fn set_address_register(machine: &mut Machine, address: usize) {
    machine.registers.address_register = address;
}

pub fn get_register(machine: &Machine, index: usize) -> u8 {
    machine.registers.general_registers[index]
}

pub fn set_register(machine: &mut Machine, index: usize, val: u8) {
    machine.registers.general_registers[index] = val;
}

pub fn read_memory(machine: &Machine, address: usize) -> u8 {
    machine.ram.memory[address]
}

pub fn write_memory(machine: &mut Machine, address: usize, val: u8) {
    if address >= START_USER_SPACE && address < MEM_SIZE {
        machine.ram.memory[address] = val
    } else {
        panic!("memory address out of range")
    }
}

fn write_protected_space(machine: &mut Machine, address: usize, val: u8) {
    machine.ram.memory[address] = val
}

pub fn get_key(machine: &Machine, key: usize) -> bool {
    machine.keys[key]
}

pub fn set_key(machine: &mut Machine, key: usize, state: bool) {
    machine.keys[key] = state;
}

pub fn get_delay_timer(machine: &Machine) -> u8 {
    machine.timers.delay_timer
}

pub fn set_delay_timer(machine: &mut Machine, delay: u8) {
    machine.timers.delay_timer = delay;
}

pub fn get_sound_timer(machine: &Machine) -> u8 {
    machine.timers.sound_timer
}

pub fn set_sound_timer(machine: &mut Machine, delay: u8) {
    machine.timers.sound_timer = delay;
}

pub fn get_flag(machine: &Machine) -> Flags {
    let flag = machine.flag.clone();
    return flag;
}

pub fn set_flag(machine: &mut Machine, flag: Flags) {
    machine.flag = flag;
}

pub fn get_target_register(machine: &Machine) -> usize {
    machine.target_register
}

pub fn set_target_register(machine: &mut Machine, register: usize) {
    machine.target_register = register;
}

pub fn get_screenbuffer(machine: &mut Machine) -> &mut render::ScreenBuffer {
    &mut machine.screen_buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation_ram_is_zeroes() {
        let machine = create_machine();
        for i in START_USER_SPACE .. MEM_SIZE {
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
    fn creation_program_counter() {
        let machine = create_machine();
        assert_eq!(START_USER_SPACE, machine.program_counter);
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

    #[test]
    fn increment_pc() {
        let mut machine = create_machine();
        increment_program_counter(&mut machine);
        assert_eq!(START_USER_SPACE + 2, get_program_counter(&machine));
    }

    #[test]
    fn jump_to_address() {
        let mut machine = create_machine();
        let address: usize = 0x300;
        jump(&mut machine, address);

        assert_eq!(address, get_program_counter(&machine));
        assert_eq!(0, machine.stack.len());
    }
    #[test]
    fn call_address() {
        let mut machine = create_machine();
        let address: usize = 0x300;
        call(&mut machine, address);
        
        assert_eq!(address, get_program_counter(&machine));
        assert_eq!(1, machine.stack.len());
        assert_eq!(START_USER_SPACE + 2, pop_stack(&mut machine));
    }

    #[test]
    fn ret_from_address() {
        let mut machine = create_machine();
        let address: usize = 0x300;
        call(&mut machine, address);
        ret(&mut machine);

        assert_eq!(START_USER_SPACE + 2, get_program_counter(&machine));
        assert_eq!(0, machine.stack.len());
    }

    #[test]
    fn test_set_register() {
        let mut machine = create_machine();
        set_register(&mut machine, 0, 0x8);
        assert_eq!(0x8, get_register(&machine, 0));
    }

    #[test]
    #[should_panic]
    fn test_set_regist_out_of_range() {
        let mut machine = create_machine();
        set_register(&mut machine, 16, 1);
    }

    #[test]
    #[should_panic]
    fn test_get_regist_out_of_range() {
        let mut machine = create_machine();
        get_register(&mut machine, 16);
    }

    #[test]
    fn test_write_read_memory() {
        let mut machine = create_machine();
        write_memory(&mut machine, 0x400, 0x8);
        assert_eq!(0x8, read_memory(&machine, 0x400));
    }

    #[test]
    fn test_write_read_protected_space() {
        let mut machine = create_machine();
        write_protected_space(&mut machine, 0x100, 0x8);
        assert_eq!(0x8, read_memory(&machine, 0x100));
    }

    #[test]
    #[should_panic]
    fn test_write_to_protected_space_panic() {
        let mut machine = create_machine();
        write_memory(&mut machine, 0x100, 0x8);
    }

    #[test]
    #[should_panic]
    fn test_write_memory_out_of_range() {
        let mut machine = create_machine();
        write_memory(&mut machine, MEM_SIZE, 0x8);
    }

    #[test]
    #[should_panic]
    fn test_read_memory_out_of_range() {
        let mut machine = create_machine();
        read_memory(&machine, MEM_SIZE);
    }
}
