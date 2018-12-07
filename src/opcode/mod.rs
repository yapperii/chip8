
pub struct OpCode {
    opcode: u16,
}

pub fn create_opcode(a: u8, b: u8) -> OpCode {
    let mut combined: u16 = a as u16;
    let b16 = b as u16;
    combined = (combined << 8) | b16;
    let opcode = OpCode{opcode: combined };
    return opcode;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation_combined() {
        let opcode = create_opcode(1, 1);
        assert_eq!(257, opcode.opcode);
    }
}