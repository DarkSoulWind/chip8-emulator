#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Register {
    PC = 0, // Program counter is 16 bits
    IR = 2, // Index register is 16 bits
    V0 = 4,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    VF,
    DELAY_TIMER
}

impl Register {
    pub fn v_register_from(number: u8) -> Register {
        match number {
            0 => Register::V0,
            1 => Register::V1,
            2 => Register::V2,
            3 => Register::V3,
            4 => Register::V4,
            5 => Register::V5,
            6 => Register::V6,
            7 => Register::V7,
            8 => Register::V8,
            9 => Register::V9,
            10 => Register::VA,
            11 => Register::VB,
            12 => Register::VC,
            13 => Register::VD,
            14 => Register::VE,
            15 => Register::VF,
            _ => panic!("Invalid v register number")
        }
    }
}
