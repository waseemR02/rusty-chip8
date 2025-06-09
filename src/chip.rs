use crate::instructions::Instruction;

const MEM_SIZE: usize = 4096;
const START_MEM: u16 = 0x200;

pub struct Chip {
    pub v: [u8; 16],
    pub i: u16,
    pub sp: u16,
    pub st: u8,
    pub dt: u8,
    pub pc: u16,
    pub mem: [u8; MEM_SIZE],
}

impl Default for Chip {
    fn default() -> Self {
        Self::new()
    }
}

impl Chip {
    pub fn new() -> Chip {
        Chip {
            v: [0; 16],
            i: 0,
            sp: 0,
            st: 0,
            dt: 0,
            pc: START_MEM,
            mem: [0; MEM_SIZE],
        }
    }

    pub fn load(&mut self, filepath: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = std::fs::read(filepath)?;
        if (buffer.len() & 1) == 1 {
            buffer.push(0x00);
        }

        for (dst, src) in self.mem[(START_MEM as usize)..].iter_mut().zip(&buffer) {
            *dst = *src
        }

        Ok(())
    }

    pub fn interpret(&mut self, instruction: Instruction) {
        match instruction.f_nibble {
            0x0 => {
                if instruction.x == 0x00 {
                    match instruction.nn {
                        0xE0 => self.cls(),
                        0xEE => self.not_implemented(),
                        _ => eprintln!("UNKNOWN 0"),
                    }
                } else {
                    eprintln!("UNKNOWN 0");
                }
            }
            0x1 => self.jump(instruction),
            0x6 => self.mvi(instruction),
            0x7 => self.adi(instruction),
            0xA => self.mvi(instruction),
            0xD => self.draw(instruction),
            _ => todo!(),
        }
    }

    fn cls(&mut self) {
        // TODO: Add clear screen logic
        self.pc += 0x02;
    }
    fn jump(&mut self, instruction: Instruction) {
        self.pc = instruction.nnn;
    }

    fn mov(&mut self, instruction: Instruction) {
        match instruction.f_nibble {
            0x8 => self.not_implemented(),
            0xF => self.not_implemented(),
            _ => eprintln!("UNKNOWN MOV"),
        }
        self.pc += 0x02;
    }

    fn mvi(&mut self, instruction: Instruction) {
        match instruction.f_nibble {
            0x6 => self.v[instruction.x as usize] = instruction.nn,
            0xA => self.i = instruction.nnn,
            _ => eprintln!("UNKNOWN MVI"),
        }
        self.pc += 0x02;
    }

    fn adi(&mut self, instruction: Instruction) {
        match instruction.f_nibble {
            0x7 => self.v[instruction.x as usize] += instruction.nn,
            0xF => self.not_implemented(),
            _ => eprintln!("UNKNOWN ADI"),
        }
        self.pc += 0x02;
    }

    fn draw(&mut self, instruction: Instruction) {
        todo!();
    }

    fn not_implemented(&mut self) {
        println!("Opcode not implemented yet!.");
        self.pc += 0x02;
    }
}

#[cfg(test)]
mod tests {

    use super::{Chip, Instruction};

    #[test]
    fn test_jump() {
        let mut chip8 = Chip::new();
        chip8.interpret(Instruction::new(&[0x12, 0x28]));

        assert_eq!(chip8.pc, 0x228);
    }

    #[test]
    fn test_mvi_op6() {
        let mut chip8 = Chip::new();

        chip8.interpret(Instruction::new(&[0x60, 0x0C]));

        assert_eq!(chip8.v[0], 0x0C);
    }

    #[test]
    fn test_mvi_opa() {
        let mut chip8 = Chip::new();

        chip8.interpret(Instruction::new(&[0xA2, 0x2A]));

        assert_eq!(chip8.i, 0x22A);
    }

    #[test]
    fn test_adi_op7() {
        let mut chip8 = Chip::new();

        chip8.interpret(Instruction::new(&[0x70, 0x09]));

        assert_eq!(chip8.v[0], 0x09);
    }
}
