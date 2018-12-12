use machine;

#[derive(Debug, Clone)]
pub enum Code_Mask {
    ZERO_NNN = 0x0000,
    ZERO_ZERO_E_ZERO = 0x00e0,
    ZERO_ZERO_EE = 0x00ee,
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
        OpCode {raw: 0, code_mask: Code_Mask::ZERO_NNN, x_mask: 0x0, y_mask:0x0, n_mask: 0x0fff}, 
        OpCode {raw: 0, code_mask: Code_Mask::ZERO_ZERO_E_ZERO, x_mask: 0x0, y_mask:0x0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::ZERO_ZERO_EE, x_mask: 0x0, y_mask:0x0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::ONE_NNN, x_mask: 0x0, y_mask:0x0, n_mask: 0x0fff},
        OpCode {raw: 0, code_mask: Code_Mask::TWO_NNN, x_mask: 0x0, y_mask:0x0, n_mask: 0x0fff},
        OpCode {raw: 0, code_mask: Code_Mask::THREE_XNN, x_mask: 0x0f00, y_mask:0x0, n_mask: 0x00ff},
        OpCode {raw: 0, code_mask: Code_Mask::FOUR_XNN, x_mask: 0x0f00, y_mask:0x0, n_mask: 0x00ff},
        OpCode {raw: 0, code_mask: Code_Mask::FIVE_XY_ZERO, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::SIX_XNN, x_mask: 0x0f00, y_mask:0x0, n_mask: 0x00ff},
        OpCode {raw: 0, code_mask: Code_Mask::SEVEN_XNN, x_mask: 0x0f00, y_mask:0x0, n_mask: 0x00ff},
        OpCode {raw: 0, code_mask: Code_Mask::EIGHT_XY_ZERO, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::EIGHT_XY_ONE, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::EIGHT_XY_TWO, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::EIGHT_XY_THREE, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::EIGHT_XY_FOUR, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::EIGHT_XY_FIVE, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::EIGHT_XY_SIX, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::EIGHT_XY_SEVEN, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0},
        OpCode {raw: 0, code_mask: Code_Mask::EIGHT_XY_E, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0},
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
        OpCode {raw: 0, code_mask: Code_Mask::FX_SIX_FIVE, x_mask: 0x0f00, y_mask: 0x0, n_mask: 0x0},
    ]}
}

fn identify_opcode(val: u16, lib: &OpCode_Lib) -> usize {
    for i in (0..NUM_OPCODES).rev() {
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

pub fn extract_value(raw: u16, mask: u16) -> u16 {
    let bits: u16 = raw & mask;
    let trailing = mask.trailing_zeros();
    bits.wrapping_shr(trailing)
}

fn op_0NNN(opcode: OpCode, mach: &mut machine::Machine) {
    // probably not needed
}

// clears the screen
fn op_00E0(opcode: OpCode, mach: &mut machine::Machine) {
    //not implemented
}

// returns from a function
fn op_00EE(opcode: OpCode, mach: &mut machine::Machine) {
    let pc = machine::pop_stack(mach);
    machine::set_program_counter(mach, pc);
}

pub fn execute_opcode(opcode: &OpCode, mach: &mut machine::Machine) {

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
        assert_eq!(0, identify_opcode(0x0123, &lib));
        assert_eq!(1, identify_opcode(0x01e0, &lib));
        assert_eq!(2, identify_opcode(0x01ee, &lib));   
        assert_eq!(2, identify_opcode(0x02ee, &lib));
        assert_eq!(3, identify_opcode(0x1234, &lib));
        assert_eq!(4, identify_opcode(0x2345, &lib));
        assert_eq!(5, identify_opcode(0x3456, &lib));
        assert_eq!(6, identify_opcode(0x4567, &lib));
        assert_eq!(7, identify_opcode(0x5670, &lib));
        assert_eq!(8, identify_opcode(0x6789, &lib));
        assert_eq!(9, identify_opcode(0x789a, &lib));
        assert_eq!(10, identify_opcode(0x89a0, &lib));
        assert_eq!(11, identify_opcode(0x89a1, &lib));
        assert_eq!(12, identify_opcode(0x89a2, &lib));
        assert_eq!(13, identify_opcode(0x89a3, &lib));
        assert_eq!(14, identify_opcode(0x89a4, &lib));
        assert_eq!(15, identify_opcode(0x89a5, &lib));
        assert_eq!(16, identify_opcode(0x89a6, &lib));
        assert_eq!(17, identify_opcode(0x89a7, &lib));
        assert_eq!(18, identify_opcode(0x89ae, &lib));
        assert_eq!(19, identify_opcode(0x9ab0, &lib));
        assert_eq!(20, identify_opcode(0xabcd, &lib));
        assert_eq!(21, identify_opcode(0xbcde, &lib));
        assert_eq!(22, identify_opcode(0xcdef, &lib));
        assert_eq!(23, identify_opcode(0xdef0, &lib));
        assert_eq!(24, identify_opcode(0xef9e, &lib));
        assert_eq!(25, identify_opcode(0xefa1, &lib));
        assert_eq!(26, identify_opcode(0xf107, &lib));
        assert_eq!(27, identify_opcode(0xf10a, &lib));
        assert_eq!(28, identify_opcode(0xf115, &lib));
        assert_eq!(29, identify_opcode(0xf118, &lib));
        assert_eq!(30, identify_opcode(0xf11e, &lib));
        assert_eq!(31, identify_opcode(0xf129, &lib));
        assert_eq!(32, identify_opcode(0xf133, &lib));
        assert_eq!(33, identify_opcode(0xf155, &lib));
        assert_eq!(34, identify_opcode(0xf165, &lib));
    }

    #[test]
    fn extract_values() {
        let lib = create_opcode_lib();
        {
            let opcode = create_opcode(0x01, 0x23, &lib);
            assert_eq!(0x0123, extract_value(opcode.raw, opcode.n_mask));
        }
        {
            let opcode = create_opcode(0x12, 0x34, &lib);
            assert_eq!(0x0234, extract_value(opcode.raw, opcode.n_mask));
        }
        {
            let opcode = create_opcode(0x23, 0x45, &lib);
            assert_eq!(0x0345, extract_value(opcode.raw, opcode.n_mask));
        }
        {
            let opcode = create_opcode(0x34, 0x56, &lib);
            assert_eq!(0x0004, extract_value(opcode.raw, opcode.x_mask));
            assert_eq!(0x0056, extract_value(opcode.raw, opcode.n_mask));
        }
        {
            let opcode = create_opcode(0x45, 0x67, &lib);
            assert_eq!(0x0005, extract_value(opcode.raw, opcode.x_mask));
            assert_eq!(0x0067, extract_value(opcode.raw, opcode.n_mask));
        }
        {
            let opcode = create_opcode(0x56, 0x78, &lib);
            assert_eq!(0x0006, extract_value(opcode.raw, opcode.x_mask));
            assert_eq!(0x0007, extract_value(opcode.raw, opcode.y_mask));
        }
        {
            let opcode = create_opcode(0x67, 0x89, &lib);
            assert_eq!(0x0007, extract_value(opcode.raw, opcode.x_mask));
            assert_eq!(0x0089, extract_value(opcode.raw, opcode.n_mask));
        }
        {
            let opcode = create_opcode(0x78, 0x9a, &lib);
            assert_eq!(0x0008, extract_value(opcode.raw, opcode.x_mask));
            assert_eq!(0x009a, extract_value(opcode.raw, opcode.n_mask));
        }
        {
            let opcode = create_opcode(0x89, 0xa0, &lib);
            assert_eq!(0x0009, extract_value(opcode.raw, opcode.x_mask));
            assert_eq!(0x000a, extract_value(opcode.raw, opcode.y_mask));
        }
        {
            let opcode = create_opcode(0x89, 0xa1, &lib);
            assert_eq!(0x0009, extract_value(opcode.raw, opcode.x_mask));
            assert_eq!(0x000a, extract_value(opcode.raw, opcode.y_mask));
        }
        {
            let opcode = create_opcode(0x89, 0xa2, &lib);
            assert_eq!(0x0009, extract_value(opcode.raw, opcode.x_mask));
            assert_eq!(0x000a, extract_value(opcode.raw, opcode.y_mask));
        }
        {
            let opcode = create_opcode(0x89, 0xa3, &lib);
            assert_eq!(0x0009, extract_value(opcode.raw, opcode.x_mask));
            assert_eq!(0x000a, extract_value(opcode.raw, opcode.y_mask));
        }
        {
            let opcode = create_opcode(0x89, 0xa4, &lib);
            assert_eq!(0x0009, extract_value(opcode.raw, opcode.x_mask));
            assert_eq!(0x000a, extract_value(opcode.raw, opcode.y_mask));
        }
        {
            let opcode = create_opcode(0x89, 0xa5, &lib);
            assert_eq!(0x0009, extract_value(opcode.raw, opcode.x_mask));
            assert_eq!(0x000a, extract_value(opcode.raw, opcode.y_mask));
        }
        {
            let opcode = create_opcode(0x89, 0xa6, &lib);
            assert_eq!(0x0009, extract_value(opcode.raw, opcode.x_mask));
            assert_eq!(0x000a, extract_value(opcode.raw, opcode.y_mask));
        }
        {
            let opcode = create_opcode(0x89, 0xa7, &lib);
            assert_eq!(0x0009, extract_value(opcode.raw, opcode.x_mask));
            assert_eq!(0x000a, extract_value(opcode.raw, opcode.y_mask));
        }
        {
            let opcode = create_opcode(0x89, 0xae, &lib);
            assert_eq!(0x0009, extract_value(opcode.raw, opcode.x_mask));
            assert_eq!(0x000a, extract_value(opcode.raw, opcode.y_mask));
        }
        {
            let opcode = create_opcode(0x9a, 0xb0, &lib);
            assert_eq!(0x000a, extract_value(opcode.raw, opcode.x_mask));
            assert_eq!(0x000b, extract_value(opcode.raw, opcode.y_mask));
        }
        {
            let opcode = create_opcode(0xab, 0xcd, &lib);
            assert_eq!(0x0bcd, extract_value(opcode.raw, opcode.n_mask));
        }
        {
            let opcode = create_opcode(0xbc, 0xde, &lib);
            assert_eq!(0x0cde, extract_value(opcode.raw, opcode.n_mask));
        }
        {
            let opcode = create_opcode(0xcd, 0xef, &lib);
            assert_eq!(0x000d, extract_value(opcode.raw, opcode.x_mask));
            assert_eq!(0x00ef, extract_value(opcode.raw, opcode.n_mask));
        }
        {
            let opcode = create_opcode(0xde, 0xf1, &lib);
            assert_eq!(0x000e, extract_value(opcode.raw, opcode.x_mask));
            assert_eq!(0x000f, extract_value(opcode.raw, opcode.y_mask));
            assert_eq!(0x0001, extract_value(opcode.raw, opcode.n_mask));
        }
        {
            let opcode = create_opcode(0xef, 0x9e, &lib);
            assert_eq!(0x000f, extract_value(opcode.raw, opcode.x_mask));
        }
        {
            let opcode = create_opcode(0xef,0xa1, &lib);
            assert_eq!(0x000f, extract_value(opcode.raw, opcode.x_mask));
        }
        {
            let opcode = create_opcode(0xf1, 0x07, &lib);
            assert_eq!(0x0001, extract_value(opcode.raw, opcode.x_mask));
        }
        {
            let opcode = create_opcode(0xf1, 0x0a, &lib);
            assert_eq!(0x0001, extract_value(opcode.raw, opcode.x_mask));
        }
        {
            let opcode = create_opcode(0xf1, 0x15, &lib);
            assert_eq!(0x0001, extract_value(opcode.raw, opcode.x_mask));
        }
        {
            let opcode = create_opcode(0xf1, 0x18, &lib);
            assert_eq!(0x0001, extract_value(opcode.raw, opcode.x_mask));
        }
        {
            let opcode = create_opcode(0xf1, 0x1e, &lib);
            assert_eq!(0x0001, extract_value(opcode.raw, opcode.x_mask));
        }
        {
            let opcode = create_opcode(0xf1, 0x29, &lib);
            assert_eq!(0x0001, extract_value(opcode.raw, opcode.x_mask));
        }
        {
            let opcode = create_opcode(0xf1, 0x33, &lib);
            assert_eq!(0x0001, extract_value(opcode.raw, opcode.x_mask));
        }
        {
            let opcode = create_opcode(0xf1, 0x55, &lib);
            assert_eq!(0x0001, extract_value(opcode.raw, opcode.x_mask));
        }
        {
            let opcode = create_opcode(0xf1, 0x65, &lib);
            assert_eq!(0x0001, extract_value(opcode.raw, opcode.x_mask));
        }
    }
}
