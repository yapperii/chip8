use machine;
use operations;

#[derive(Debug, Clone)]
pub enum CodeMask {
    ZeroNNN = 0x0000,
    ZeroZeroEZero = 0x00e0,
    ZeroZeroEE = 0x00ee,
    OneNNN = 0x1000,
    TwoNNN = 0x2000,
    ThreeXNN = 0x3000,
    FourXNN = 0x4000,
    FiveXYZero = 0x5000,
    SixXNN = 0x6000,
    SevenXNN = 0x7000,
    EightXYOne = 0x8001,
    EightXYTwo = 0x8002,
    EightXYThree = 0x8003,
    EightXYFour = 0x8004,
    EightXYFive = 0x8005,
    EightXYSix = 0x8006,
    EightXYSeven = 0x8007,
    EightXYE = 0x800e,
    EightXYZero = 0x8000,
    NineXYZero = 0x9000,
    ANNN = 0xa000,
    BNNN = 0xb000,
    CXNN = 0xc000,
    DXYN = 0xd000,
    EXNineE = 0xe09e,
    EXAOne = 0xe0a1,
    FXZeroSeven = 0xf007,
    FXZeroA = 0xf00a,
    FXOneFive = 0xf015,
    FXOneEight = 0xf018,
    FXOneE = 0xf01e,
    FXTwoNine = 0xf029,
    FXThreeThree = 0xf033,
    FXFiveFive = 0xf055,
    FXSixFive = 0xf065,
}

pub struct OpCodePrototype
{
    code_mask: CodeMask,
    n_mask: u16,
    x_mask: u16,
    y_mask: u16,
    operation: Box<dyn Fn(&mut machine::Machine, u16, u16, u16)>,
}

const NUM_OPCODES: usize = 35;

pub struct OpCodeLib {
    code_array: [OpCodePrototype; NUM_OPCODES],
}

pub fn create_op_code_lib() -> OpCodeLib {
    OpCodeLib {code_array: [
        OpCodePrototype {code_mask: CodeMask::ZeroNNN, x_mask: 0x0, y_mask:0x0, n_mask: 0x0fff, operation: Box::new(operations::op_0nnn)}, 
        OpCodePrototype {code_mask: CodeMask::ZeroZeroEZero, x_mask: 0x0, y_mask:0x0, n_mask: 0x0, operation: Box::new(operations::op_00e0)},
        OpCodePrototype {code_mask: CodeMask::ZeroZeroEE, x_mask: 0x0, y_mask:0x0, n_mask: 0x0, operation: Box::new(operations::op_00ee)},
        OpCodePrototype {code_mask: CodeMask::OneNNN, x_mask: 0x0, y_mask:0x0, n_mask: 0x0fff, operation: Box::new(operations::op_1nnn)},
        OpCodePrototype {code_mask: CodeMask::TwoNNN, x_mask: 0x0, y_mask:0x0, n_mask: 0x0fff, operation: Box::new(operations::op_2nnn)},
        OpCodePrototype {code_mask: CodeMask::ThreeXNN, x_mask: 0x0f00, y_mask:0x0, n_mask: 0x00ff, operation: Box::new(operations::op_3xnn)},
        OpCodePrototype {code_mask: CodeMask::FourXNN, x_mask: 0x0f00, y_mask:0x0, n_mask: 0x00ff, operation: Box::new(operations::op_4xnn)},
        OpCodePrototype {code_mask: CodeMask::FiveXYZero, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0, operation: Box::new(operations::op_5xy0)},
        OpCodePrototype {code_mask: CodeMask::SixXNN, x_mask: 0x0f00, y_mask:0x0, n_mask: 0x00ff, operation: Box::new(operations::op_6xnn)},
        OpCodePrototype {code_mask: CodeMask::SevenXNN, x_mask: 0x0f00, y_mask:0x0, n_mask: 0x00ff, operation: Box::new(operations::op_7xnn)},
        OpCodePrototype {code_mask: CodeMask::EightXYZero, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0, operation: Box::new(operations::op_8xy0)},
        OpCodePrototype {code_mask: CodeMask::EightXYOne, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0, operation: Box::new(operations::op_8xy1)},
        OpCodePrototype {code_mask: CodeMask::EightXYTwo, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0, operation: Box::new(operations::op_8xy2)},
        OpCodePrototype {code_mask: CodeMask::EightXYThree, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0, operation: Box::new(operations::op_8xy3)},
        OpCodePrototype {code_mask: CodeMask::EightXYFour, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0, operation: Box::new(operations::op_8xy4)},
        OpCodePrototype {code_mask: CodeMask::EightXYFive, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0, operation: Box::new(operations::op_8xy5)},
        OpCodePrototype {code_mask: CodeMask::EightXYSix, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0, operation: Box::new(operations::op_8xy6)},
        OpCodePrototype {code_mask: CodeMask::EightXYSeven, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0, operation: Box::new(operations::op_8xy7)},
        OpCodePrototype {code_mask: CodeMask::EightXYE, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0, operation: Box::new(operations::op_8xye)},
        OpCodePrototype {code_mask: CodeMask::NineXYZero, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x0, operation: Box::new(operations::op_9xy0)},
        OpCodePrototype {code_mask: CodeMask::ANNN, x_mask: 0x0, y_mask:0x0, n_mask: 0x0fff, operation: Box::new(operations::op_annn)},
        OpCodePrototype {code_mask: CodeMask::BNNN, x_mask: 0x0, y_mask:0x0, n_mask: 0x0fff, operation: Box::new(operations::op_bnnn)},
        OpCodePrototype {code_mask: CodeMask::CXNN, x_mask: 0x0f00, y_mask:0x0, n_mask: 0x00ff, operation: Box::new(operations::op_cxnn)},
        OpCodePrototype {code_mask: CodeMask::DXYN, x_mask: 0x0f00, y_mask:0x00f0, n_mask: 0x000f, operation: Box::new(operations::op_dxyn)},
        OpCodePrototype {code_mask: CodeMask::EXNineE, x_mask: 0x0f00, y_mask: 0x0, n_mask: 0x0, operation: Box::new(operations::op_ex9e)},
        OpCodePrototype {code_mask: CodeMask::EXAOne, x_mask: 0x0f00, y_mask: 0x0, n_mask: 0x0, operation: Box::new(operations::op_exa1)},
        OpCodePrototype {code_mask: CodeMask::FXZeroSeven, x_mask: 0x0f00, y_mask: 0x0, n_mask: 0x0, operation: Box::new(operations::op_fx07)},
        OpCodePrototype {code_mask: CodeMask::FXZeroA, x_mask: 0x0f00, y_mask: 0x0, n_mask: 0x0, operation: Box::new(operations::op_fx0a)},
        OpCodePrototype {code_mask: CodeMask::FXOneFive, x_mask: 0x0f00, y_mask: 0x0, n_mask: 0x0, operation: Box::new(operations::op_fx15)},
        OpCodePrototype {code_mask: CodeMask::FXOneEight, x_mask: 0x0f00, y_mask: 0x0, n_mask: 0x0, operation: Box::new(operations::op_fx18)},
        OpCodePrototype {code_mask: CodeMask::FXOneE, x_mask: 0x0f00, y_mask: 0x0, n_mask: 0x0, operation: Box::new(operations::op_fx1e)},
        OpCodePrototype {code_mask: CodeMask::FXTwoNine, x_mask: 0x0f00, y_mask: 0x0, n_mask: 0x0, operation: Box::new(operations::op_fx29)},
        OpCodePrototype {code_mask: CodeMask::FXThreeThree, x_mask: 0x0f00, y_mask: 0x0, n_mask: 0x0, operation: Box::new(operations::op_fx33)},
        OpCodePrototype {code_mask: CodeMask::FXFiveFive, x_mask: 0x0f00, y_mask: 0x0, n_mask: 0x0, operation: Box::new(operations::op_fx55)},
        OpCodePrototype {code_mask: CodeMask::FXSixFive, x_mask: 0x0f00, y_mask: 0x0, n_mask: 0x0, operation: Box::new(operations::op_fx65)},
    ]}
}

fn identify_opcode(val: u16, lib: &OpCodeLib) -> usize {
    for i in (0..NUM_OPCODES).rev() {
        let code: u16 = lib.code_array[i].code_mask.clone() as u16;
        let masked: u16 = val & code;
        if masked == code {
            return i;
        }
    }

    panic!("unknown opcode");
}

pub fn extract_value(raw: u16, mask: u16) -> u16 {
    let bits: u16 = raw & mask;
    let trailing = mask.trailing_zeros();
    bits.wrapping_shr(trailing)
}

pub fn decode_and_execute(mach: &mut machine::Machine, a: u8, b: u8, lib: &OpCodeLib) {
    let mut combined: u16 = a as u16;
    let b16 = b as u16;
    combined = (combined << 8) | b16;
    let opcode_index = identify_opcode(combined, lib);

    let x: u16 = extract_value(combined, lib.code_array[opcode_index].x_mask);
    let y: u16 = extract_value(combined, lib.code_array[opcode_index].y_mask);
    let n: u16 = extract_value(combined, lib.code_array[opcode_index].n_mask);
    (lib.code_array[opcode_index].operation)(mach, x, y, n);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn identify_opcodes() {
        let lib = create_op_code_lib();
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
        let lib = create_op_code_lib();
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
