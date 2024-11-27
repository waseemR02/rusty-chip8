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
