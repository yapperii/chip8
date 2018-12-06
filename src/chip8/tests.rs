mod chip8;

#[cfg(test)]
mod tests {

    #[test]
    fn creation_stack_capacity() {
        machine: chip8::Machine = chip8::create_machine()();
        assert_eq!(0, machine.stack.capacity());
    }
}