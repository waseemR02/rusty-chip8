pub struct Instruction {
    pub f_nibble: u8,
    pub l_nibble: u8,
    pub nnn: u16,
    pub x: u8,
    pub y: u8,
    pub nn: u8,
    pub opcode: u16,
}

impl Instruction {
    pub fn new(hex_code: &[u8]) -> Instruction {
        Instruction {
            f_nibble: hex_code[0] >> 4,
            l_nibble: hex_code[1] & 0xF,
            nnn: (hex_code[0] as u16 & 0xF) << 8 | hex_code[1] as u16,
            x: hex_code[0] & 0xF,
            y: hex_code[1] >> 4,
            nn: hex_code[1],
            opcode: (hex_code[0] as u16) << 8 | hex_code[1] as u16,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Instruction;

    #[test]
    fn test_nnn() {
        let instruct = Instruction::new(&[0x12, 0x28]);

        assert_eq!(instruct.opcode, 0x1228);
        assert_eq!(instruct.nnn, 0x228);
    }

    #[test]
    fn test_x() {
        let instruct = Instruction::new(&[0x43, 0xE0]);

        assert_eq!(instruct.opcode, 0x43E0);
        assert_eq!(instruct.x, 0x3);
    }

    #[test]
    fn test_y() {
        let instruct = Instruction::new(&[0x51, 0x20]);

        assert_eq!(instruct.opcode, 0x5120);
        assert_eq!(instruct.y, 0x2);
    }

    #[test]
    fn test_n() {
        let instruct = Instruction::new(&[0xD3, 0x42]);

        assert_eq!(instruct.opcode, 0xD342);
        assert_eq!(instruct.l_nibble, 0x2);
    }

    #[test]
    fn test_f_nibble() {
        let instruct = Instruction::new(&[0xD3, 0x42]);

        assert_eq!(instruct.opcode, 0xD342);
        assert_eq!(instruct.f_nibble, 0xD);
    }

    #[test]
    fn test_nn() {
        let instruct = Instruction::new(&[0x32, 0x54]);

        assert_eq!(instruct.opcode, 0x3254);
        assert_eq!(instruct.nn, 0x54);
    }
}
