use colored::*;
use std::fs;

use crate::instructions::Instruction;

pub fn disasm(filepath: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = fs::read(&filepath)?;
    if (buffer.len() & 1) == 1 {
        buffer.push(0x00);
    }
    let mut pc: u16 = 0x200;
    let mut iter = buffer.chunks(2);

    println!("Disassembly of {}:\n", &filepath);
    for _ in (0..buffer.len()).step_by(2) {
        let instruction = Instruction::new(iter.next().unwrap());
        decode(&instruction, pc);
        pc += 0x02;
    }
    Ok(())
}

pub fn decode(instruct: &Instruction, pc: u16) {
    print!("  {pc:04X}:\t\t {:04X}\t", instruct.opcode);

    match instruct.f_nibble {
        0x0 => {
            if instruct.opcode >> 12 == 0x00 {
                match instruct.nn {
                    0xE0 => println!("{:<10}", "CLS".yellow()),
                    0xEE => println!("{:<10}", "RTS".yellow()),
                    _ => println!("{}", "UNKNOWN 0".red()),
                }
            } else {
                println!("{}", "UNKNOWN 0".red())
            }
        }
        0x1 => println!("{:<10} ${:03X}", "JUMP".yellow(), instruct.nnn,),
        0x2 => println!("{:<10} ${:03X}", "CALL".yellow(), instruct.nnn,),
        0x3 => println!(
            "{:<10} V{:X}, #${:02X}",
            "SKIP.EQ".yellow(),
            instruct.x,
            instruct.nn
        ),
        0x4 => println!(
            "{:<10} V{:X}, #${:02X}",
            "SKIP.NE".yellow(),
            instruct.x,
            instruct.nn
        ),
        0x5 => println!(
            "{:<10} V{:X}, V{:X}",
            "SKIP.EQ".yellow(),
            instruct.x,
            instruct.y
        ),
        0x6 => println!(
            "{:<10} V{:X}, #${:02X}",
            "MVI".yellow(),
            instruct.x,
            instruct.nn
        ),
        0x7 => println!(
            "{:<10} V{:X}, #${:02X}",
            "ADI".yellow(),
            instruct.x,
            instruct.nn
        ),
        0x8 => match instruct.l_nibble {
            0x0 => println!(
                "{:<10} V{:X}, V{:X}",
                "MOV".yellow(),
                instruct.x,
                instruct.y,
            ),
            0x1 => println!("{:<10} V{:X}, V{:X}", "OR".yellow(), instruct.x, instruct.y,),
            0x2 => println!(
                "{:<10} V{:X}, V{:X}",
                "AND".yellow(),
                instruct.x,
                instruct.y,
            ),
            0x3 => println!(
                "{:<10} V{:X}, V{:X}",
                "XOR".yellow(),
                instruct.x,
                instruct.y,
            ),
            0x4 => println!(
                "{:<10} V{:X}, V{:X}",
                "ADD.".yellow(),
                instruct.x,
                instruct.y,
            ),
            0x5 => println!(
                "{:<10} V{:X}, V{:X}",
                "SUB.".yellow(),
                instruct.x,
                instruct.y,
            ),
            0x6 => println!("{:<10} V{:X}", "SHR.".yellow(), instruct.x,),
            0x7 => println!(
                "{:<10} V{:X}, V{:X}",
                "SUBN.".yellow(),
                instruct.x,
                instruct.y,
            ),
            0xE => println!("{:<10} V{:X}", "SHL.".yellow(), instruct.x,),
            _ => println!("{}", "UNKNOWN 8".red()),
        },
        0x9 => println!(
            "{:<10} V{:X}, V{:X}",
            "SKIP.NE".yellow(),
            instruct.x,
            instruct.y,
        ),
        0xA => println!("{:<10} I, #${:03X}", "MVI".yellow(), instruct.nnn,),
        0xB => println!("{:<10} #${:03X}(V0)", "JUMP".yellow(), instruct.nnn,),
        0xC => println!(
            "{:<10} V{:X}, #${:02X}",
            "RNDMSK".yellow(),
            instruct.x,
            instruct.nn
        ),
        0xD => println!(
            "{:<10} V{:X}, V{:X}, #${:X}",
            "DRAW".yellow(),
            instruct.x,
            instruct.y,
            instruct.l_nibble
        ),
        0xE => match instruct.nn {
            0x9E => println!("{:<10} V{:X}", "SKIPKEY.Y.yellow()", instruct.x),
            0xA1 => println!("{:<10} V{:X}", "SKIPKEY.N.yellow()", instruct.x),
            _ => println!("{}", "UNKNOWN E".red()),
        },
        0xF => match instruct.nn {
            0x07 => println!("{:<10} V{:X}, DELAY", "MOV".yellow(), instruct.x),
            0x0A => println!("{:<10} V{:X}", "KEY".yellow(), instruct.x),
            0x15 => println!("{:<10} DELAY, V{:X}", "MOV".yellow(), instruct.x),
            0x18 => println!("{:<10} SOUND, V{:X}", "MOV".yellow(), instruct.x),
            0x1E => println!("{:<10} I, V{:X}", "ADI".yellow(), instruct.x),
            0x29 => println!("{:<10} I, V{:X}", "SPRITECHAR".yellow(), instruct.x),
            0x33 => println!("{:<10} (I), V{:X}", "MOVBCD".yellow(), instruct.x),
            0x55 => println!("{:<10} (I), V0-V{:X}", "MOVM".yellow(), instruct.x),
            0x65 => println!("{:<10} V0-V{:X}, (I)", "MOVM".yellow(), instruct.x),
            _ => println!("{}", "UNKNOWN F".red()),
        },
        _ => println!("{}", "UNKNOWN I".red()),
    }
}
