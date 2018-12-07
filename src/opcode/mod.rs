#[derive(Debug)]
pub enum Code {
    ZERO_ZERO_EE = 0x00ee,
    ZERO_ZERO_E_ZERO = 0x00e0,
    ZERO_NNN = 0x0000,
    ONE_NNN = 0x1000,
    TWO_NNN = 0x2000,
    THREE_XNN = 0x3000,
    FOUR_XNN = 0x4000,
    FIVE_XY_ZERO = 0x5000,
    SIX_XNN = 0x6000,
    SEVEN_XNN = 0x7000,
    EIGHT_XY_ONE = 0x8001,
    EIGHT_XY_TWO = 0x8002,
    EIGHT_XY_THREE = 0x8003,
    EIGHT_XY_FOUR = 0x8004,
    EIGHT_XY_FIVE = 0x8005,
    EIGHT_XY_SIX = 0x8006,
    EIGHT_XY_SEVEN = 0x8007,
    EIGHT_XY_E = 0x800e,
    EIGHT_XY_ZERO = 0x8000,
    NINE_XY_ZERO = 0x9000,
    ANNN = 0xa000,
    BNNN = 0xb000,
    CXNN = 0xc000,
    DXYN = 0xd000,
    EX_NINE_E = 0xe09e,
    EXA_ONE = 0xe0a1,
    FX_ZERO_SEVEN = 0xf007,
    FX_ZERO_A = 0xf00a,
    FX_ONE_FIVE = 0xf015,
    FX_ONE_EIGHT = 0xf018,
    FX_ONE_E = 0xf01e,
    FX_TWO_NINE = 0xf029,
    FX_THREE_THREE = 0xf033,
    FX_FIVE_FIVE = 0xf055,
    FX_SIX_FIVE = 0xf065,
}
const NUM_OPCODES: usize = 35;
const CODE_ARRAY: [Code; NUM_OPCODES] = [Code::ZERO_ZERO_EE, Code::ZERO_ZERO_E_ZERO, Code::ZERO_NNN, Code::ONE_NNN,
                                         Code::TWO_NNN, Code::THREE_XNN, Code::FOUR_XNN, Code::FIVE_XY_ZERO, Code::SIX_XNN,
                                         Code::SEVEN_XNN, Code::EIGHT_XY_ONE, Code::EIGHT_XY_TWO, Code::EIGHT_XY_THREE,
                                         Code::EIGHT_XY_FOUR, Code::EIGHT_XY_FIVE, Code::EIGHT_XY_SIX, Code::EIGHT_XY_SEVEN,
                                         Code::EIGHT_XY_E, Code::EIGHT_XY_ZERO, Code::NINE_XY_ZERO, Code::ANNN, Code::BNNN,
                                         Code::CXNN, Code::DXYN, Code::EX_NINE_E, Code::EXA_ONE, Code::FX_ZERO_SEVEN,
                                         Code::FX_ZERO_A, Code::FX_ONE_FIVE, Code::FX_ONE_EIGHT, Code::FX_ONE_E,
                                         Code::FX_TWO_NINE, Code::FX_THREE_THREE, Code::FX_FIVE_FIVE, Code::FX_SIX_FIVE];
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

pub fn identify_opcode(val: u16) -> Code {
    for i in 0..NUM_OPCODES {
        let code: u16 = CODE_ARRAY[i].Copy() as u16;
        let masked: u16 = val & code;
        if masked == code {
            return CODE_ARRAY[i];
        }
    }

    panic!("unknown opcode");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation_combined() {
        let opcode = create_opcode(1, 1);
        assert_eq!(257, opcode.opcode);
    }

    #[test]
    fn identify_opcodes() {
        assert_eq!(Code::ZERO_ZERO_EE, identify_opcode(0x00EE));
        assert_eq!(Code::ZERO_ZERO_EE, identify_opcode(0x05EE));
    }
}