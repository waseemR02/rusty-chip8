use std::fmt::Display;

use minifb::{Key, Window};

use crate::{dump, instructions::Instruction};

const MEM_SIZE: usize = 4096;
const START_MEM: u16 = 0x200;

pub struct Chip {
    pub v: [u8; 16],
    pub i: u16,
    pub stack: [u16; 8],
    pub sp: usize,
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
        writeln!(f, "   SP: {}", self.sp)?;
        writeln!(f, "   Stack: ")?;
        self.stack
            .iter()
            .try_for_each(|s| writeln!(f, "      {:X}", s))?;
        writeln!(f, "   Sound Timer: {:X}", self.st)?;
        writeln!(f, "   Delay Timer: {:X}", self.dt)?;
        writeln!(f, "   PC: {:X}", self.pc)?;
        self.v
            .iter()
            .enumerate()
            .try_for_each(|(i, v)| writeln!(f, "   V{:X}: {:X}", i, v))?;
        write!(f, "--------------")
    }
}

impl Chip {
    pub fn new() -> Chip {
        Chip {
            v: [0; 16],
            i: 0,
            stack: [0; 8],
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

    pub fn interpret(&mut self, instruction: Instruction, buffer: &mut [u32], window: &Window) {
        dump::decode(&instruction, self.pc);
        match instruction.f_nibble {
            0x0 => {
                if instruction.x == 0x00 {
                    match instruction.nn {
                        0xE0 => self.cls(buffer),
                        0xEE => self.rts(),
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
            0x3 => self.skip_eq(instruction),
            0x5 => self.skip_eq(instruction),
            0x4 => self.skip_ne(instruction),
            0x9 => self.skip_ne(instruction),
            0x2 => self.call(instruction),
            0x8 => self.eight_inst(instruction),
            0xC => self.rndmsk(instruction),
            0xF => self.f_inst(instruction, &window),
            0xE => self.skipkey(instruction, window),
            _ => todo!(),
        }
        if self.dt > 0 {
            self.dt -= 1
        }
        if self.st > 0 {
            self.st -= 1
        }
    }

    fn cls(&mut self, buffer: &mut [u32]) {
        buffer.fill(0u32);
        self.pc += 0x02;
    }
    fn jump(&mut self, instruction: Instruction) {
        self.pc = instruction.nnn;
    }

    fn eight_inst(&mut self, instruction: Instruction) {
        match instruction.l_nibble {
            0x0 => self.v[instruction.x as usize] = self.v[instruction.y as usize],
            0x1 => self.v[instruction.x as usize] |= self.v[instruction.y as usize],
            0x2 => self.v[instruction.x as usize] &= self.v[instruction.y as usize],
            0x3 => self.v[instruction.x as usize] ^= self.v[instruction.y as usize],
            0x4 => {
                self.v[instruction.x as usize] = {
                    self.v[instruction.x as usize]
                        .checked_add(self.v[instruction.y as usize])
                        .unwrap_or_else(|| {
                            self.v[0xF] = 1;
                            self.v[instruction.x as usize] + self.v[instruction.y as usize]
                        })
                }
            }
            0x5 => {
                self.v[instruction.x as usize] = {
                    self.v[0xF] = 1;
                    self.v[instruction.x as usize]
                        .checked_sub(self.v[instruction.y as usize])
                        .unwrap_or_else(|| {
                            self.v[0xF] = 0;
                            self.v[instruction.x as usize] - self.v[instruction.y as usize]
                        })
                }
            }
            0x6 => {
                #[cfg(not(feature = "shift"))]
                {
                    self.v[instruction.x as usize] = self.v[instruction.y as usize];
                }
                self.v[0xF] = self.v[instruction.x as usize] & 1;
                self.v[instruction.x as usize] >>= 1;
            }
            0x7 => {
                self.v[instruction.x as usize] = {
                    self.v[0xF] = 1;
                    self.v[instruction.y as usize]
                        .checked_sub(self.v[instruction.x as usize])
                        .unwrap_or_else(|| {
                            self.v[0xF] = 0;
                            self.v[instruction.y as usize] - self.v[instruction.x as usize]
                        })
                }
            }
            0xE => {
                #[cfg(not(feature = "shift"))]
                {
                    self.v[instruction.x as usize] = self.v[instruction.y as usize];
                }
                self.v[0xF] = self.v[instruction.x as usize] & (1 << 7);
                self.v[instruction.x as usize] <<= 1;
            }
            _ => eprintln!("UNKNOWN 8"),
        }
        self.pc += 0x02
    }

    fn f_inst(&mut self, instruction: Instruction, window: &Window) {
        match instruction.nn {
            0x1E => {
                self.adi(instruction);
                return;
            }
            0x07 => self.v[instruction.x as usize] = self.dt,
            0x15 => self.dt = self.v[instruction.x as usize],
            0x18 => self.st = self.v[instruction.x as usize],
            0x0A => {
                if let Some(key) = window.get_keys().first() {
                    self.v[instruction.x as usize] = Keypad(*key).into();
                } else {
                    self.pc -= 0x02;
                }
            }
            0x55 => (0..=instruction.x)
                .for_each(|x| self.mem[(self.i + x as u16) as usize] = self.v[x as usize]),
            0x65 => (0..=instruction.x)
                .for_each(|x| self.v[x as usize] = self.mem[(self.i + x as u16) as usize]),
            _ => todo!(),
        }
        self.pc += 0x02
    }

    fn rndmsk(&mut self, instruction: Instruction) {
        self.v[instruction.x as usize] = fastrand::u8(..) & instruction.nn;
        self.pc += 0x02
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
            0xF => self.i += self.v[instruction.x as usize] as u16,
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

    fn skip_eq(&mut self, instruction: Instruction) {
        match instruction.f_nibble {
            0x3 => {
                if self.v[instruction.x as usize] == instruction.nn {
                    self.pc += 0x02
                }
            }
            0x5 => {
                if self.v[instruction.x as usize] == self.v[instruction.y as usize] {
                    self.pc += 0x02
                }
            }
            _ => eprintln!("UNKNOWN SKIP.EQ"),
        }
        self.pc += 0x02
    }

    fn skip_ne(&mut self, instruction: Instruction) {
        match instruction.f_nibble {
            0x4 => {
                if self.v[instruction.x as usize] != instruction.nn {
                    self.pc += 0x02
                }
            }
            0x9 => {
                if self.v[instruction.x as usize] != self.v[instruction.y as usize] {
                    self.pc += 0x02
                }
            }
            _ => eprintln!("UNKNOWN SKIP.NE"),
        }
        self.pc += 0x02
    }

    fn skipkey(&mut self, instruction: Instruction, window: &Window) {
        let keypad: Keypad = self.v[instruction.x as usize].into();
        match instruction.nn {
            0x9E => {
                if window.is_key_down(keypad.0) {
                    self.pc += 0x02
                }
            }
            0xA1 => {
                if !window.is_key_down(keypad.0) {
                    println!("Not pressed {}", self.v[instruction.x as usize]);
                    self.pc += 0x02
                }
            }
            _ => eprintln!("UKNOWN E"),
        }
        self.pc += 0x02
    }

    fn call(&mut self, instruction: Instruction) {
        self.stack[self.sp] = self.pc + 0x2;
        self.sp += 1;
        self.pc = instruction.nnn;
    }

    fn rts(&mut self) {
        self.pc = self.stack[self.sp - 1];
        self.stack[self.sp - 1] = 0u16;
        self.sp -= 1;
    }

    fn not_implemented(&mut self) {
        println!("Opcode not implemented yet!.");
        self.pc += 0x02;
    }
}

pub struct Keypad(Key);

impl From<u8> for Keypad {
    fn from(value: u8) -> Self {
        match value {
            0x0 => Keypad(Key::Key1),
            0x1 => Keypad(Key::Key2),
            0x2 => Keypad(Key::Key3),
            0x3 => Keypad(Key::Key4),
            0x4 => Keypad(Key::Q),
            0x5 => Keypad(Key::W),
            0x6 => Keypad(Key::E),
            0x7 => Keypad(Key::R),
            0x8 => Keypad(Key::A),
            0x9 => Keypad(Key::S),
            0xA => Keypad(Key::D),
            0xB => Keypad(Key::F),
            0xC => Keypad(Key::Z),
            0xD => Keypad(Key::X),
            0xE => Keypad(Key::C),
            0xF => Keypad(Key::V),
            _ => panic!("can't convert illegal key {}", value),
        }
    }
}

impl From<Keypad> for u8 {
    fn from(value: Keypad) -> Self {
        match value.0 {
            Key::Key1 => 0x0,
            Key::Key2 => 0x1,
            Key::Key3 => 0x2,
            Key::Key4 => 0x3,
            Key::Q => 0x4,
            Key::W => 0x5,
            Key::E => 0x6,
            Key::R => 0x7,
            Key::A => 0x8,
            Key::S => 0x9,
            Key::D => 0xA,
            Key::F => 0xB,
            Key::Z => 0xC,
            Key::X => 0xD,
            Key::C => 0xE,
            Key::V => 0xF,
            _ => panic!("can't convert illegal key {:#?}", value.0),
        }
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
    use minifb::{Window, WindowOptions};

    #[test]
    fn test_jump() {
        let mut chip8 = Chip::new();
        let mut buffer = vec![0u32, 64 * 32];
        chip8.interpret(
            Instruction::new(&[0x12, 0x28]),
            &mut buffer,
            &Window::new("Test", 1, 1, WindowOptions::default()).unwrap(),
        );

        assert_eq!(chip8.pc, 0x228);
    }

    #[test]
    fn test_mvi_op6() {
        let mut chip8 = Chip::new();

        let mut buffer = vec![0u32, 64 * 32];
        chip8.interpret(
            Instruction::new(&[0x60, 0x0C]),
            &mut buffer,
            &Window::new("Test", 1, 1, WindowOptions::default()).unwrap(),
        );

        assert_eq!(chip8.v[0], 0x0C);
    }

    #[test]
    fn test_mvi_opa() {
        let mut chip8 = Chip::new();

        let mut buffer = vec![0u32, 64 * 32];
        chip8.interpret(
            Instruction::new(&[0xA2, 0x2A]),
            &mut buffer,
            &Window::new("Test", 1, 1, WindowOptions::default()).unwrap(),
        );

        assert_eq!(chip8.i, 0x22A);
    }

    #[test]
    fn test_adi_op7() {
        let mut chip8 = Chip::new();

        let mut buffer = vec![0u32, 64 * 32];
        chip8.interpret(
            Instruction::new(&[0x70, 0x09]),
            &mut buffer,
            &Window::new("Test", 1, 1, WindowOptions::default()).unwrap(),
        );

        assert_eq!(chip8.v[0], 0x09);
    }
}
