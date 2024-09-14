use crate::register::Register;

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    CLS,
    JP(u16),                     // JP NNN
    LDImm(Register, u8),         // LD Vx, NN (Set Vx = NN)
    ADDImm(Register, u8),        // ADD Vx, NN (Set Vx = Vx + NN)
    LDDir(Register, Register),   // LD Vx, Vy (Set Vx = Vy)
    OR(Register, Register),      // OR Vx, Vy (Set Vx = Vx OR Vy)
    AND(Register, Register),     // AND Vx, Vy (Set Vx = Vx AND Vy)
    XOR(Register, Register),     // XOR Vx, Vy (Set Vx = Vx XOR Vy)
    ADDDir(Register, Register),  // ADD Vx, Vy (Set Vx = Vx + Vy, set VF = carry)
    LDI(u16),                    // LD I, NNN (Set I = NNN)
    DRW(Register, Register, u8), // DRW Vx, Vy, N
    LDK(Register, u8),               // LD Vx, K
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

        return match n1 {
            1 => Instruction::JP(((n2 as u16) << 8) | (b2 as u16)),
            6 => Instruction::LDImm(vx, b2),
            7 => Instruction::ADDImm(vx, b2),
            8 => match n4 {
                0 => Instruction::LDDir(vx, vy),
                1 => Instruction::OR(vx, vy),
                2 => Instruction::AND(vx, vy),
                3 => Instruction::XOR(vx, vy),
                4 => Instruction::ADDDir(vx, vy),
                _ => panic!("8 ERM WHAT THE FUCKING SIGMA"),
            },
            0xA => Instruction::LDI(((n2 as u16) << 8) | (b2 as u16)),
            0xD => Instruction::DRW(vx, vy, n4),
            _ => {
                panic!("ERM WHAT THE FUCKING SIGMA")
            }
        };
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
}
