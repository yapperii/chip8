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

pub struct OpCodePrototype {
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
    OpCodeLib {
        code_array: [
            OpCodePrototype {
                code_mask: CodeMask::ZeroNNN,
                x_mask: 0x0,
                y_mask: 0x0,
                n_mask: 0x0fff,
                operation: Box::new(operations::op_0nnn),
            },
            OpCodePrototype {
                code_mask: CodeMask::ZeroZeroEZero,
                x_mask: 0x0,
                y_mask: 0x0,
                n_mask: 0x0,
                operation: Box::new(operations::op_00e0),
            },
            OpCodePrototype {
                code_mask: CodeMask::ZeroZeroEE,
                x_mask: 0x0,
                y_mask: 0x0,
                n_mask: 0x0,
                operation: Box::new(operations::op_00ee),
            },
            OpCodePrototype {
                code_mask: CodeMask::OneNNN,
                x_mask: 0x0,
                y_mask: 0x0,
                n_mask: 0x0fff,
                operation: Box::new(operations::op_1nnn),
            },
            OpCodePrototype {
                code_mask: CodeMask::TwoNNN,
                x_mask: 0x0,
                y_mask: 0x0,
                n_mask: 0x0fff,
                operation: Box::new(operations::op_2nnn),
            },
            OpCodePrototype {
                code_mask: CodeMask::ThreeXNN,
                x_mask: 0x0f00,
                y_mask: 0x0,
                n_mask: 0x00ff,
                operation: Box::new(operations::op_3xnn),
            },
            OpCodePrototype {
                code_mask: CodeMask::FourXNN,
                x_mask: 0x0f00,
                y_mask: 0x0,
                n_mask: 0x00ff,
                operation: Box::new(operations::op_4xnn),
            },
            OpCodePrototype {
                code_mask: CodeMask::FiveXYZero,
                x_mask: 0x0f00,
                y_mask: 0x00f0,
                n_mask: 0x0,
                operation: Box::new(operations::op_5xy0),
            },
            OpCodePrototype {
                code_mask: CodeMask::SixXNN,
                x_mask: 0x0f00,
                y_mask: 0x0,
                n_mask: 0x00ff,
                operation: Box::new(operations::op_6xnn),
            },
            OpCodePrototype {
                code_mask: CodeMask::SevenXNN,
                x_mask: 0x0f00,
                y_mask: 0x0,
                n_mask: 0x00ff,
                operation: Box::new(operations::op_7xnn),
            },
            OpCodePrototype {
                code_mask: CodeMask::EightXYZero,
                x_mask: 0x0f00,
                y_mask: 0x00f0,
                n_mask: 0x0,
                operation: Box::new(operations::op_8xy0),
            },
            OpCodePrototype {
                code_mask: CodeMask::EightXYOne,
                x_mask: 0x0f00,
                y_mask: 0x00f0,
                n_mask: 0x0,
                operation: Box::new(operations::op_8xy1),
            },
            OpCodePrototype {
                code_mask: CodeMask::EightXYTwo,
                x_mask: 0x0f00,
                y_mask: 0x00f0,
                n_mask: 0x0,
                operation: Box::new(operations::op_8xy2),
            },
            OpCodePrototype {
                code_mask: CodeMask::EightXYThree,
                x_mask: 0x0f00,
                y_mask: 0x00f0,
                n_mask: 0x0,
                operation: Box::new(operations::op_8xy3),
            },
            OpCodePrototype {
                code_mask: CodeMask::EightXYFour,
                x_mask: 0x0f00,
                y_mask: 0x00f0,
                n_mask: 0x0,
                operation: Box::new(operations::op_8xy4),
            },
            OpCodePrototype {
                code_mask: CodeMask::EightXYFive,
                x_mask: 0x0f00,
                y_mask: 0x00f0,
                n_mask: 0x0,
                operation: Box::new(operations::op_8xy5),
            },
            OpCodePrototype {
                code_mask: CodeMask::EightXYSix,
                x_mask: 0x0f00,
                y_mask: 0x00f0,
                n_mask: 0x0,
                operation: Box::new(operations::op_8xy6),
            },
            OpCodePrototype {
                code_mask: CodeMask::EightXYSeven,
                x_mask: 0x0f00,
                y_mask: 0x00f0,
                n_mask: 0x0,
                operation: Box::new(operations::op_8xy7),
            },
            OpCodePrototype {
                code_mask: CodeMask::EightXYE,
                x_mask: 0x0f00,
                y_mask: 0x00f0,
                n_mask: 0x0,
                operation: Box::new(operations::op_8xye),
            },
            OpCodePrototype {
                code_mask: CodeMask::NineXYZero,
                x_mask: 0x0f00,
                y_mask: 0x00f0,
                n_mask: 0x0,
                operation: Box::new(operations::op_9xy0),
            },
            OpCodePrototype {
                code_mask: CodeMask::ANNN,
                x_mask: 0x0,
                y_mask: 0x0,
                n_mask: 0x0fff,
                operation: Box::new(operations::op_annn),
            },
            OpCodePrototype {
                code_mask: CodeMask::BNNN,
                x_mask: 0x0,
                y_mask: 0x0,
                n_mask: 0x0fff,
                operation: Box::new(operations::op_bnnn),
            },
            OpCodePrototype {
                code_mask: CodeMask::CXNN,
                x_mask: 0x0f00,
                y_mask: 0x0,
                n_mask: 0x00ff,
                operation: Box::new(operations::op_cxnn),
            },
            OpCodePrototype {
                code_mask: CodeMask::DXYN,
                x_mask: 0x0f00,
                y_mask: 0x00f0,
                n_mask: 0x000f,
                operation: Box::new(operations::op_dxyn),
            },
            OpCodePrototype {
                code_mask: CodeMask::EXNineE,
                x_mask: 0x0f00,
                y_mask: 0x0,
                n_mask: 0x0,
                operation: Box::new(operations::op_ex9e),
            },
            OpCodePrototype {
                code_mask: CodeMask::EXAOne,
                x_mask: 0x0f00,
                y_mask: 0x0,
                n_mask: 0x0,
                operation: Box::new(operations::op_exa1),
            },
            OpCodePrototype {
                code_mask: CodeMask::FXZeroSeven,
                x_mask: 0x0f00,
                y_mask: 0x0,
                n_mask: 0x0,
                operation: Box::new(operations::op_fx07),
            },
            OpCodePrototype {
                code_mask: CodeMask::FXZeroA,
                x_mask: 0x0f00,
                y_mask: 0x0,
                n_mask: 0x0,
                operation: Box::new(operations::op_fx0a),
            },
            OpCodePrototype {
                code_mask: CodeMask::FXOneFive,
                x_mask: 0x0f00,
                y_mask: 0x0,
                n_mask: 0x0,
                operation: Box::new(operations::op_fx15),
            },
            OpCodePrototype {
                code_mask: CodeMask::FXOneEight,
                x_mask: 0x0f00,
                y_mask: 0x0,
                n_mask: 0x0,
                operation: Box::new(operations::op_fx18),
            },
            OpCodePrototype {
                code_mask: CodeMask::FXOneE,
                x_mask: 0x0f00,
                y_mask: 0x0,
                n_mask: 0x0,
                operation: Box::new(operations::op_fx1e),
            },
            OpCodePrototype {
                code_mask: CodeMask::FXTwoNine,
                x_mask: 0x0f00,
                y_mask: 0x0,
                n_mask: 0x0,
                operation: Box::new(operations::op_fx29),
            },
            OpCodePrototype {
                code_mask: CodeMask::FXThreeThree,
                x_mask: 0x0f00,
                y_mask: 0x0,
                n_mask: 0x0,
                operation: Box::new(operations::op_fx33),
            },
            OpCodePrototype {
                code_mask: CodeMask::FXFiveFive,
                x_mask: 0x0f00,
                y_mask: 0x0,
                n_mask: 0x0,
                operation: Box::new(operations::op_fx55),
            },
            OpCodePrototype {
                code_mask: CodeMask::FXSixFive,
                x_mask: 0x0f00,
                y_mask: 0x0,
                n_mask: 0x0,
                operation: Box::new(operations::op_fx65),
            },
        ],
    }
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
