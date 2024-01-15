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

impl Machine {
    pub fn new() -> Self {
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
                screen_buffer: render::ScreenBuffer::new()};
    
        mach.init_font();
        return mach;
    }

fn init_font(&mut self) {
    // zero
    self.write_protected_space(START_FONT + 0x0, 0xf0);
    self.write_protected_space(START_FONT + 0x1, 0x90);
    self.write_protected_space(START_FONT + 0x2, 0x90);
    self.write_protected_space(START_FONT + 0x3, 0x90);
    self.write_protected_space(START_FONT + 0x4, 0xf0);

    // one
    self.write_protected_space(START_FONT + 0x5, 0x20);
    self.write_protected_space(START_FONT + 0x6, 0x60);
    self.write_protected_space(START_FONT + 0x7, 0x20);
    self.write_protected_space(START_FONT + 0x8, 0x20);
    self.write_protected_space(START_FONT + 0x9, 0x70);

    // two
    self.write_protected_space(START_FONT + 0xa, 0xf0);
    self.write_protected_space(START_FONT + 0xb, 0x10);
    self.write_protected_space(START_FONT + 0xc, 0xf0);
    self.write_protected_space(START_FONT + 0xd, 0x80);
    self.write_protected_space(START_FONT + 0xe, 0xf0);

    // three
    self.write_protected_space(START_FONT + 0xf, 0xf0);
    self.write_protected_space(START_FONT + 0x10, 0x10);
    self.write_protected_space(START_FONT + 0x11, 0xf0);
    self.write_protected_space(START_FONT + 0x12, 0x10);
    self.write_protected_space(START_FONT + 0x13, 0xf0);

    // four
    self.write_protected_space(START_FONT + 0x14, 0x90);
    self.write_protected_space(START_FONT + 0x15, 0x90);
    self.write_protected_space(START_FONT + 0x16, 0xf0);
    self.write_protected_space(START_FONT + 0x17, 0x10);
    self.write_protected_space(START_FONT + 0x18, 0x10);

    // five
    self.write_protected_space(START_FONT + 0x19, 0xf0);
    self.write_protected_space(START_FONT + 0x1a, 0x80);
    self.write_protected_space(START_FONT + 0x1b, 0xf0);
    self.write_protected_space(START_FONT + 0x1c, 0x10);
    self.write_protected_space(START_FONT + 0x1d, 0xf0);

    // six
    self.write_protected_space(START_FONT + 0x1e, 0xf0);
    self.write_protected_space(START_FONT + 0x1f, 0x80);
    self.write_protected_space(START_FONT + 0x20, 0xf0);
    self.write_protected_space(START_FONT + 0x21, 0x90);
    self.write_protected_space(START_FONT + 0x22, 0xf0);

    // seven
    self.write_protected_space(START_FONT + 0x23, 0xf0);
    self.write_protected_space(START_FONT + 0x24, 0x10);
    self.write_protected_space(START_FONT + 0x25, 0x20);
    self.write_protected_space(START_FONT + 0x26, 0x40);
    self.write_protected_space(START_FONT + 0x27, 0x40);

    // eight
    self.write_protected_space(START_FONT + 0x28, 0xf0);
    self.write_protected_space(START_FONT + 0x29, 0x90);
    self.write_protected_space(START_FONT + 0x2a, 0xf0);
    self.write_protected_space(START_FONT + 0x2b, 0x90);
    self.write_protected_space(START_FONT + 0x2c, 0xf0);

    // 9
    self.write_protected_space(START_FONT + 0x2d, 0xf0);
    self.write_protected_space(START_FONT + 0x2e, 0x90);
    self.write_protected_space(START_FONT + 0x2f, 0xf0);
    self.write_protected_space(START_FONT + 0x30, 0x10);
    self.write_protected_space(START_FONT + 0x31, 0xf0);

    // a
    self.write_protected_space(START_FONT + 0x32, 0xf0);
    self.write_protected_space(START_FONT + 0x33, 0x90);
    self.write_protected_space(START_FONT + 0x34, 0xf0);
    self.write_protected_space(START_FONT + 0x35, 0x90);
    self.write_protected_space(START_FONT + 0x36, 0x90);

    // b
    self.write_protected_space(START_FONT + 0x37, 0xe0);
    self.write_protected_space(START_FONT + 0x38, 0x90);
    self.write_protected_space(START_FONT + 0x39, 0xe0);
    self.write_protected_space(START_FONT + 0x3a, 0x90);
    self.write_protected_space(START_FONT + 0x3b, 0xe0);

    // c
    self.write_protected_space(START_FONT + 0x3c, 0xf0);
    self.write_protected_space(START_FONT + 0x3d, 0x80);
    self.write_protected_space(START_FONT + 0x3e, 0x80);
    self.write_protected_space(START_FONT + 0x3f, 0x80);
    self.write_protected_space(START_FONT + 0x40, 0xf0);

    // d
    self.write_protected_space(START_FONT + 0x41, 0xe0);
    self.write_protected_space(START_FONT + 0x42, 0x90);
    self.write_protected_space(START_FONT + 0x43, 0x90);
    self.write_protected_space(START_FONT + 0x44, 0x90);
    self.write_protected_space(START_FONT + 0x45, 0xe0);

    // e
    self.write_protected_space(START_FONT + 0x46, 0xf0);
    self.write_protected_space(START_FONT + 0x47, 0x80);
    self.write_protected_space(START_FONT + 0x48, 0xf0);
    self.write_protected_space(START_FONT + 0x49, 0x80);
    self.write_protected_space(START_FONT + 0x4a, 0xf0);

    // f
    self.write_protected_space(START_FONT + 0x4b, 0xf0);
    self.write_protected_space(START_FONT + 0x4c, 0x80);
    self.write_protected_space(START_FONT + 0x4d, 0xf0);
    self.write_protected_space(START_FONT + 0x4e, 0x80);
    self.write_protected_space(START_FONT + 0x4f, 0x80);
}

fn push_stack(&mut self, val: usize) {
    if self.stack.len() >= STACK_SIZE {
        panic!("stack overrun!")
    }

    self.stack.push(val);
}

fn pop_stack(&mut self) -> usize {
    let popped = self.stack.pop();
    match popped {
        Some(x) => x,
        None => panic!("stack underrun!")
    }
}

#[allow(dead_code)]
pub fn peek_stack(&self) -> Option<usize> {
    match self.stack.len() {
        0 => None,
        n => Some(self.stack[n-1])
    }
}

pub fn get_program_counter(&self) -> usize {
    self.program_counter
}

fn set_program_counter(&mut self, counter: usize) {
    if counter >= START_USER_SPACE  && counter < MEM_SIZE {
        self.program_counter = counter;
    }else
    {
        panic!("program counter out of range");
    }
}

pub fn increment_program_counter(&mut self) {
    self.program_counter += 2;
}

pub fn jump(&mut self, address: usize) {
    self.set_program_counter(address);
}

pub fn call(&mut self, address: usize) {
    self.increment_program_counter();
    let pc = self.get_program_counter();
    self.push_stack(pc);
    self.set_program_counter(address);
}

pub fn ret(&mut self) {
    let pc = self.pop_stack();
    self.set_program_counter(pc);
}

pub fn get_address_register(&self) -> usize {
    self.registers.address_register
}

pub fn set_address_register(&mut self, address: usize) {
    self.registers.address_register = address;
}

pub fn get_register(&self, index: usize) -> u8 {
    self.registers.general_registers[index]
}

pub fn set_register(&mut self, index: usize, val: u8) {
    self.registers.general_registers[index] = val;
}

pub fn read_memory(&self, address: usize) -> u8 {
    self.ram.memory[address]
}

pub fn write_memory(&mut self, address: usize, val: u8) {
    if address >= START_USER_SPACE && address < MEM_SIZE {
        self.ram.memory[address] = val
    } else {
        panic!("memory address out of range")
    }
}

fn write_protected_space(&mut self, address: usize, val: u8) {
    self.ram.memory[address] = val
}

pub fn get_key(&self, key: usize) -> bool {
    self.keys[key]
}

pub fn set_key(&mut self, key: usize, state: bool) {
    self.keys[key] = state;
}

pub fn get_delay_timer(&self) -> u8 {
    self.timers.delay_timer
}

pub fn set_delay_timer(&mut self, delay: u8) {
    self.timers.delay_timer = delay;
}

pub fn get_sound_timer(&self) -> u8 {
    self.timers.sound_timer
}

pub fn set_sound_timer(&mut self, delay: u8) {
    self.timers.sound_timer = delay;
}

pub fn get_flag(&self) -> Flags {
    let flag = self.flag.clone();
    return flag;
}

pub fn set_flag(&mut self, flag: Flags) {
    self.flag = flag;
}

pub fn get_target_register(&self) -> usize {
    self.target_register
}

pub fn set_target_register(&mut self, register: usize) {
    self.target_register = register;
}

pub fn get_screenbuffer(&mut self) -> &mut render::ScreenBuffer {
    &mut self.screen_buffer
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation_ram_is_zeroes() {
        let machine = Machine::new();
        for i in START_USER_SPACE .. MEM_SIZE {
            assert_eq!(0, machine.ram.memory[i])
        }
    }

    #[test]
    fn creation_registers_are_zeroes() {
        let machine = Machine::new();
        for i in 0 .. NUM_REGISTERS {
            assert_eq!(0, machine.registers.general_registers[i])
        }

        assert_eq!(0, machine.registers.address_register);
    }

    #[test]
    fn creation_stack_len() {
        let machine = Machine::new();
        assert_eq!(0, machine.stack.len());
    }

    #[test]
    fn creation_program_counter() {
        let machine = Machine::new();
        assert_eq!(START_USER_SPACE, machine.program_counter);
    }

    #[test]
    fn push_pop() {
        let mut machine = Machine::new();
        machine.push_stack(1);
        assert_eq!(1, machine.pop_stack());

        machine.push_stack(1);
        machine.push_stack(2);
        machine.push_stack(3);

        assert_eq!(3, machine.stack.len());

        assert_eq!(3, machine.pop_stack());
        assert_eq!(2, machine.pop_stack());
        assert_eq!(1, machine.pop_stack());

        assert_eq!(0, machine.stack.len());
    }

    #[test]
    #[should_panic]
    fn stack_overflow() {
        let mut machine = Machine::new();
        for _i in 0 .. (STACK_SIZE + 1) {
            machine.push_stack(0);
        }
    }

    #[test]
    #[should_panic]
    fn stack_underflow() {
        let mut machine = Machine::new();
        machine.pop_stack();
    }

    #[test]
    fn change_program_counter() {
        let mut machine = Machine::new();
        let pc: usize = START_USER_SPACE;
        machine.set_program_counter(pc);
        assert_eq!(START_USER_SPACE, machine.get_program_counter());
    }

    #[test]
    #[should_panic]
    fn change_program_counter_before_user_space() {
        let mut machine = Machine::new();
        let pc: usize = START_USER_SPACE -1;
        machine.set_program_counter(pc);
        assert_eq!(START_USER_SPACE, machine.get_program_counter());
    }

    #[test]
    #[should_panic]
    fn change_program_counter_too_high() {
        let mut machine = Machine::new();
        let pc: usize = MEM_SIZE;
        machine.set_program_counter(pc);
        assert_eq!(START_USER_SPACE, machine.get_program_counter());
    }

    #[test]
    fn increment_pc() {
        let mut machine = Machine::new();
        machine.increment_program_counter();
        assert_eq!(START_USER_SPACE + 2, machine.get_program_counter());
    }

    #[test]
    fn jump_to_address() {
        let mut machine = Machine::new();
        let address: usize = 0x300;
        machine.jump(address);

        assert_eq!(address, machine.get_program_counter());
        assert_eq!(0, machine.stack.len());
    }
    #[test]
    fn call_address() {
        let mut machine = Machine::new();
        let address: usize = 0x300;
        machine.call(address);
        
        assert_eq!(address, machine.get_program_counter());
        assert_eq!(1, machine.stack.len());
        assert_eq!(START_USER_SPACE + 2, machine.pop_stack());
    }

    #[test]
    fn ret_from_address() {
        let mut machine = Machine::new();
        let address: usize = 0x300;
        machine.call(address);
        machine.ret();

        assert_eq!(START_USER_SPACE + 2, machine.get_program_counter());
        assert_eq!(0, machine.stack.len());
    }

    #[test]
    fn test_set_register() {
        let mut machine = Machine::new();
        machine.set_register(0, 0x8);
        assert_eq!(0x8, machine.get_register(0));
    }

    #[test]
    #[should_panic]
    fn test_set_regist_out_of_range() {
        let mut machine = Machine::new();
        machine.set_register(16, 1);
    }

    #[test]
    #[should_panic]
    fn test_get_regist_out_of_range() {
        let machine = Machine::new();
        machine.get_register(16);
    }

    #[test]
    fn test_write_read_memory() {
        let mut machine = Machine::new();
        machine.write_memory(0x400, 0x8);
        assert_eq!(0x8, machine.read_memory(0x400));
    }

    #[test]
    fn test_write_read_protected_space() {
        let mut machine = Machine::new();
        machine.write_protected_space(0x100, 0x8);
        assert_eq!(0x8, machine.read_memory(0x100));
    }

    #[test]
    #[should_panic]
    fn test_write_to_protected_space_panic() {
        let mut machine = Machine::new();
        machine.write_memory(0x100, 0x8);
    }

    #[test]
    #[should_panic]
    fn test_write_memory_out_of_range() {
        let mut machine = Machine::new();
        machine.write_memory(MEM_SIZE, 0x8);
    }

    #[test]
    #[should_panic]
    fn test_read_memory_out_of_range() {
        let machine = Machine::new();
        machine.read_memory(MEM_SIZE);
    }
}
