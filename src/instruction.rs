use crate::register::Register;

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    CLS,
    JP(u16),                     // (0x1NNN) JP NNN
    SEImm(Register, u8),         // (0x3XNN) SE Vx, NN (Skip next instruction if Vx == NN)
    SNE(Register, u8),           // (0x4XNN) SNE Vx, NN (Skip next instruction if Vx != NN)
    SEDir(Register, Register),   // (0x5XY0) SE Vx, Vy (Skip next instruction if Vx == Vy)
    LDImm(Register, u8),         // (0x6XNN) LD Vx, NN (Set Vx = NN)
    ADDImm(Register, u8),        // (0x7XNN) ADD Vx, NN (Set Vx = Vx + NN)
    LDDir(Register, Register),   // (0x8XY0) LD Vx, Vy (Set Vx = Vy)
    OR(Register, Register),      // (0x8XY1) OR Vx, Vy (Set Vx = Vx OR Vy)
    AND(Register, Register),     // (0x8XY2) AND Vx, Vy (Set Vx = Vx AND Vy)
    XOR(Register, Register),     // (0x8XY3) XOR Vx, Vy (Set Vx = Vx XOR Vy)
    ADDDir(Register, Register),  // (0x8XY4) ADD Vx, Vy (Set Vx = Vx + Vy, set VF = carry)
    SUB(Register, Register),     // (0x8XY5) SUB Vx, Vy (Set Vx = Vx - Vy, set VF = NOT borrow)
    LDI(u16),                    // (0xANNN) LD I, NNN (Set I = NNN)
    JPOff(u16),                  // (0xBNNN) JP V0, NNN (Jump to address V0 + NNN)
    DRW(Register, Register, u8), // DRW Vx, Vy, N
    LDK(Register),               // (0xFX0A) LD Vx, K
}

impl Instruction {
    pub fn decode(instruction: u16) -> Instruction {
        if instruction == 0x00E0 {
            return Instruction::CLS;
        }

        /* println!("instruction: {:#4x}", instruction); */
        let [b1, b2] = instruction.to_be_bytes();
        let n1 = b1 >> 4;
        let n2 = b1 & 0xF;
        let n3 = b2 >> 4;
        let n4 = b2 & 0xF;
        let vx = Register::v_register_from(n2);
        let vy = Register::v_register_from(n3);

        match n1 {
            1 => Instruction::JP(((n2 as u16) << 8) | (b2 as u16)),
            3 => Instruction::SEImm(vx, b2),
            4 => Instruction::SNE(vx, b2),
            5 => Instruction::SEDir(vx, vy),
            6 => Instruction::LDImm(vx, b2),
            7 => Instruction::ADDImm(vx, b2),
            8 => match n4 {
                0 => Instruction::LDDir(vx, vy),
                1 => Instruction::OR(vx, vy),
                2 => Instruction::AND(vx, vy),
                3 => Instruction::XOR(vx, vy),
                4 => Instruction::ADDDir(vx, vy),
                5 => Instruction::SUB(vx, vy),
                _ => panic!("Could not decode instruction beginning with 0x8"),
            },
            0xA => Instruction::LDI(((n2 as u16) << 8) | (b2 as u16)),
            0xB => Instruction::JPOff(((n2 as u16) << 8) | (b2 as u16)),
            0xD => Instruction::DRW(vx, vy, n4),
            0xF => match b2 {
                0x0A => Instruction::LDK(Register::v_register_from(n2)),
                _ => panic!("Could not decode instruction beginning with 0xF"),
            },
            _ => {
                panic!("Could not decode instruction at all")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_ld_imm() {
        assert_eq!(
            Instruction::decode(0x6069),
            Instruction::LDImm(Register::v_register_from(0), 0x69)
        )
    }

    #[test]
    fn test_decode_add_imm() {
        assert_eq!(
            Instruction::decode(0x7069),
            Instruction::ADDImm(Register::v_register_from(0), 0x69)
        )
    }

    #[test]
    fn test_decode_ld_dir() {
        assert_eq!(
            Instruction::decode(0x8010),
            Instruction::LDDir(Register::v_register_from(0), Register::v_register_from(1))
        )
    }

    #[test]
    fn test_decode_or() {
        assert_eq!(
            Instruction::decode(0x8011),
            Instruction::OR(Register::v_register_from(0), Register::v_register_from(1))
        )
    }

    #[test]
    fn test_decode_and() {
        assert_eq!(
            Instruction::decode(0x8012),
            Instruction::AND(Register::v_register_from(0), Register::v_register_from(1))
        )
    }

    #[test]
    fn test_decode_xor() {
        assert_eq!(
            Instruction::decode(0x8013),
            Instruction::XOR(Register::v_register_from(0), Register::v_register_from(1))
        )
    }

    #[test]
    fn test_decode_ldi() {
        assert_eq!(Instruction::decode(0xA300), Instruction::LDI(0x300))
    }

    #[test]
    fn test_decode_se_imm() {
        assert_eq!(
            Instruction::decode(0x3069),
            Instruction::SEImm(Register::v_register_from(0), 0x69)
        )
    }

    #[test]
    fn test_decode_sne() {
        assert_eq!(
            Instruction::decode(0x4069),
            Instruction::SNE(Register::v_register_from(0), 0x69)
        )
    }

    #[test]
    fn test_decode_se_dir() {
        assert_eq!(
            Instruction::decode(0x5010),
            Instruction::SEDir(Register::v_register_from(0), Register::v_register_from(1))
        )
    }
}
