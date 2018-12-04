mod chip8;

#[cfg(test)]
mod tests {

    #[test]
    fn power_on_stack_capacity() {
        machine: chip8::Machine;
        chip8::power_on(&machine);
        assert_eq!(0, machine.stack.capacity());
    }
}