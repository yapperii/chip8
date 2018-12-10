#[derive(Debug, Clone)]
pub enum Code_Mask {
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

#[derive(Debug, Clone)]
pub struct OpCode
{
    raw: u16,
    code_mask: Code_Mask,
    n_mask: u16,
    x_mask: u16,
    y_mask: u16,
    // op function
}

const NUM_OPCODES: usize = 35;

pub struct OpCode_Lib {
    code_array: [OpCode; NUM_OPCODES],
}

pub fn create_opcode_lib() -> OpCode_Lib {
    OpCode_Lib {code_array: [   
        OpCode {raw: 0, code_mask: Code_Mask::ZERO_ZERO_EE, x_mask: 0x0, y_mask:0x0, n_mask: 0x0}, 
        OpCode {raw: 0, code_mask: Code_Mask::ZERO_ZERO_E_ZERO, x_mask: 0x0, y_mask:0x0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::ZERO_NNN, x_mask: 0x0, y_mask:0x0, n_mask: 0x0fff},
        OpCode {raw: 0, code_mask: Code_Mask::ONE_NNN, x_mask: 0x0, y_mask:0x0, n_mask: 0x0fff},
        OpCode {raw: 0, code_mask: Code_Mask::TWO_NNN, x_mask: 0x0, y_mask:0x0, n_mask: 0x0fff},
        OpCode {raw: 0, code_mask: Code_Mask::THREE_XNN, x_mask: 0x0f00, y_mask:0x0, n_mask: 0x00ff},
        OpCode {raw: 0, code_mask: Code_Mask::FOUR_XNN, x_mask: 0x0f00, y_mask:0x0, n_mask: 0x00ff},
        OpCode {raw: 0, code_mask: Code_Mask::FIVE_XY_ZERO, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::SIX_XNN, x_mask: 0x0f00, y_mask:0x0, n_mask: 0x00ff},
        OpCode {raw: 0, code_mask: Code_Mask::SEVEN_XNN, x_mask: 0x0f00, y_mask:0x0, n_mask: 0x00ff},
        OpCode {raw: 0, code_mask: Code_Mask::EIGHT_XY_ONE, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::EIGHT_XY_TWO, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::EIGHT_XY_THREE, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::EIGHT_XY_FOUR, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::EIGHT_XY_FIVE, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::EIGHT_XY_SIX, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::EIGHT_XY_SEVEN, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::EIGHT_XY_E, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::EIGHT_XY_ZERO, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::NINE_XY_ZERO, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::ANNN, x_mask: 0x0, y_mask:0x0, n_mask: 0x0fff},
        OpCode {raw: 0, code_mask: Code_Mask::BNNN, x_mask: 0x0, y_mask:0x0, n_mask: 0x0fff},
        OpCode {raw: 0, code_mask: Code_Mask::CXNN, x_mask: 0x0f00, y_mask:0x0, n_mask: 0x00ff},
        OpCode {raw: 0, code_mask: Code_Mask::DXYN, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x000f},
        OpCode {raw: 0, code_mask: Code_Mask::EX_NINE_E, x_mask: 0x0f00, y_mask: 0x0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::EXA_ONE, x_mask: 0x0f00, y_mask: 0x0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::FX_ZERO_SEVEN, x_mask: 0x0f00, y_mask: 0x0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::FX_ZERO_A, x_mask: 0x0f00, y_mask: 0x0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::FX_ONE_FIVE, x_mask: 0x0f00, y_mask: 0x0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::FX_ONE_EIGHT, x_mask: 0x0f00, y_mask: 0x0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::FX_ONE_E, x_mask: 0x0f00, y_mask: 0x0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::FX_TWO_NINE, x_mask: 0x0f00, y_mask: 0x0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::FX_THREE_THREE, x_mask: 0x0f00, y_mask: 0x0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::FX_FIVE_FIVE, x_mask: 0x0f00, y_mask: 0x0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::FX_SIX_FIVE, x_mask: 0x0f00, y_mask: 0x0, n_mask: 0x0}
    ]}
}

fn identify_opcode(val: u16, lib: &OpCode_Lib) -> usize {
    for i in 0..NUM_OPCODES {
        let code: u16 = lib.code_array[i].code_mask.clone() as u16;
        let masked: u16 = val & code;
        if masked == code {
            return i;
        }
    }

    panic!("unknown opcode");
}

pub fn create_opcode(a: u8, b: u8, lib: &OpCode_Lib) -> OpCode {
    let mut combined: u16 = a as u16;
    let b16 = b as u16;
    combined = (combined << 8) | b16;
    let opcode_index = identify_opcode(combined, lib);
    let opcode = OpCode{raw: combined,
                        code_mask: lib.code_array[opcode_index].code_mask.clone(),
                        n_mask: lib.code_array[opcode_index].n_mask,
                        x_mask: lib.code_array[opcode_index].x_mask,
                        y_mask: lib.code_array[opcode_index].y_mask,};
    return opcode;
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn creation_combined() {
        let lib = create_opcode_lib();
        let opcode = create_opcode(1, 2, &lib);
        assert_eq!(258, opcode.raw);
    }

    #[test]
    fn identify_opcodes() {
        let lib = create_opcode_lib();
        assert_eq!(0, identify_opcode(0x00EE, &lib));
        assert_eq!(0, identify_opcode(0x05EE, &lib));
    }
}