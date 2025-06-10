use std::fmt::Display;

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

impl Display for Chip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Chip State: ")?;
        writeln!(f, "   I: {:X}", self.i)?;
        writeln!(f, "   Stack: {:X}", self.sp)?;
        writeln!(f, "   Sound Timer: {}", self.st)?;
        writeln!(f, "   Delay Timer: {}", self.dt)?;
        writeln!(f, "   PC: {:X}", self.pc)?;
        self.v
            .iter()
            .enumerate()
            .try_for_each(|(i, v)| writeln!(f, "   V{}: {}", i, v))?;
        write!(f, "--------------")
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

    pub fn interpret(&mut self, instruction: Instruction, buffer: &mut [u32]) {
        match instruction.f_nibble {
            0x0 => {
                if instruction.x == 0x00 {
                    match instruction.nn {
                        0xE0 => self.cls(buffer),
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
            0xD => self.draw(instruction, buffer),
            _ => todo!(),
        }
    }

    fn cls(&mut self, buffer: &mut [u32]) {
        buffer.fill(0u32);
        self.pc += 0x02;
    }
    fn jump(&mut self, instruction: Instruction) {
        self.pc = instruction.nnn;
    }

    fn _mov(&mut self, instruction: Instruction) {
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

    fn draw(&mut self, instruction: Instruction, buffer: &mut [u32]) {
        let x = self.v[instruction.x as usize] & 63;
        let y = self.v[instruction.y as usize] & 31;
        self.v[0xF] = 0;
        let n = instruction.l_nibble;

        for row in 0..n {
            let sprite_data = self.mem[self.i as usize + row as usize];
            for col in 0..8 {
                let sprite_pixel = (sprite_data >> (7 - col)) & 1;
                let x = (x + col) & 63;
                let y = (y + row) & 31;

                let index = (y as usize) * 64 + x as usize;
                let screen_pixel = &mut buffer[index];

                if sprite_pixel == 1 {
                    if *screen_pixel == from_u8_rgb(255, 255, 255) {
                        self.v[0xF] = 1;
                        *screen_pixel = 0u32;
                    } else {
                        *screen_pixel = from_u8_rgb(255, 255, 255)
                    }
                }
            }
        }

        self.pc += 0x02;
    }

    fn not_implemented(&mut self) {
        println!("Opcode not implemented yet!.");
        self.pc += 0x02;
    }
}

#[inline]
fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

#[cfg(test)]
mod tests {

    use super::{Chip, Instruction};

    #[test]
    fn test_jump() {
        let mut chip8 = Chip::new();
        let mut buffer = vec![0u32, 64 * 32];
        chip8.interpret(Instruction::new(&[0x12, 0x28]), &mut buffer);

        assert_eq!(chip8.pc, 0x228);
    }

    #[test]
    fn test_mvi_op6() {
        let mut chip8 = Chip::new();

        let mut buffer = vec![0u32, 64 * 32];
        chip8.interpret(Instruction::new(&[0x60, 0x0C]), &mut buffer);

        assert_eq!(chip8.v[0], 0x0C);
    }

    #[test]
    fn test_mvi_opa() {
        let mut chip8 = Chip::new();

        let mut buffer = vec![0u32, 64 * 32];
        chip8.interpret(Instruction::new(&[0xA2, 0x2A]), &mut buffer);

        assert_eq!(chip8.i, 0x22A);
    }

    #[test]
    fn test_adi_op7() {
        let mut chip8 = Chip::new();

        let mut buffer = vec![0u32, 64 * 32];
        chip8.interpret(Instruction::new(&[0x70, 0x09]), &mut buffer);

        assert_eq!(chip8.v[0], 0x09);
    }
}
