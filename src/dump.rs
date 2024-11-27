use colored::*;
use std::fs;

pub fn disasm(filepath: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = fs::read(&filepath)?;
    if (buffer.len() & 1) == 1 {
        buffer.push(0x00);
    }
    let mut pc: u16 = 0x200;
    let mut iter = buffer.chunks(2);

    println!("Disassembly of {}:\n", &filepath);
    for _ in (0..buffer.len()).step_by(2) {
        let instruction = iter.next().unwrap();
        decode(instruction, pc);
        pc += 0x02;
    }
    Ok(())
}

pub fn decode(instruction: &[u8], pc: u16) -> () {
    print!(
        "  {pc:04X}:\t\t {:02X} {:02X}\t",
        instruction[0], instruction[1]
    );

    let nibble = instruction[0] >> 4;
    match nibble {
        0x0 => {
            if instruction[0] == 0x00 {
                match instruction[1] {
                    0xE0 => println!("{:<10}", "CLS".yellow()),
                    0xEE => println!("{:<10}", "RTS".yellow()),
                    _ => println!("{}", "UNKNOWN 0".red()),
                }
            } else {
                println!("{}", "UNKNOWN 0".red())
            }
        }
        0x1 => println!(
            "{:<10} ${:X}{:02X}",
            "JUMP".yellow(),
            instruction[0] & 0xF,
            instruction[1]
        ),
        0x2 => println!(
            "{:<10} ${:X}{:02X}",
            "CALL".yellow(),
            instruction[0] & 0xF,
            instruction[1]
        ),
        0x3 => println!(
            "{:<10} V{:X}, #${:02X}",
            "SKIP.EQ".yellow(),
            instruction[0] & 0xF,
            instruction[1]
        ),
        0x4 => println!(
            "{:<10} V{:X}, #${:02X}",
            "SKIP.NE".yellow(),
            instruction[0] & 0xF,
            instruction[1]
        ),
        0x5 => println!(
            "{:<10} V{:X}, V{:X}",
            "SKIP.EQ".yellow(),
            instruction[0] & 0xF,
            (instruction[1] & 0xF0) >> 4
        ),
        0x6 => println!(
            "{:<10} V{:X}, #${:02X}",
            "MVI".yellow(),
            instruction[0] & 0xF,
            instruction[1]
        ),
        0x7 => println!(
            "{:<10} V{:X}, #${:02X}",
            "ADI".yellow(),
            instruction[0] & 0xF,
            instruction[1]
        ),
        0x8 => match instruction[1] & 0xF {
            0x0 => println!(
                "{:<10} V{:X}, V{:X}",
                "MOV".yellow(),
                instruction[0] & 0xF,
                instruction[1] >> 4,
            ),
            0x1 => println!(
                "{:<10} V{:X}, V{:X}",
                "OR".yellow(),
                instruction[0] & 0xF,
                instruction[1] >> 4
            ),
            0x2 => println!(
                "{:<10} V{:X}, V{:X}",
                "AND".yellow(),
                instruction[0] & 0xF,
                instruction[1] >> 4
            ),
            0x3 => println!(
                "{:<10} V{:X}, V{:X}",
                "XOR".yellow(),
                instruction[0] & 0xF,
                instruction[1] >> 4
            ),
            0x4 => println!(
                "{:<10} V{:X}, V{:X}",
                "ADD.".yellow(),
                instruction[0] & 0xF,
                instruction[1] >> 4
            ),
            0x5 => println!(
                "{:<10} V{:X}, V{:X}",
                "SUB.".yellow(),
                instruction[0] & 0xF,
                instruction[1] >> 4
            ),
            0x6 => println!("{:<10} V{:X}", "SHR.".yellow(), instruction[0] & 0xF,),
            0x7 => println!(
                "{:<10} V{:X}, V{:X}",
                "SUBN.".yellow(),
                instruction[0] & 0xF,
                instruction[1] >> 4
            ),
            0xE => println!("{:<10} V{:X}", "SHL.".yellow(), instruction[0] & 0xF,),
            _ => println!("{}", "UNKNOWN 8".red()),
        },
        0x9 => println!(
            "{:<10} V{:X}, V{:X}",
            "SKIP.NE".yellow(),
            instruction[0] & 0xF,
            instruction[1] >> 4
        ),
        0xA => println!(
            "{:<10} I, #${:X}{:02X}",
            "MVI".yellow(),
            instruction[0] & 0xF,
            instruction[1]
        ),
        0xB => println!(
            "{:<10} #${:X}{:02X}(V0)",
            "JUMP".yellow(),
            instruction[0] & 0xF,
            instruction[1]
        ),
        0xC => println!(
            "{:<10} V{:X}, #${:02X}",
            "RNDMSK".yellow(),
            instruction[0] & 0xF,
            instruction[1]
        ),
        0xD => println!(
            "{:<10} V{:X}, V{:X}, #${:X}",
            "DRAW".yellow(),
            instruction[0] & 0xF,
            instruction[1] >> 4,
            instruction[1] & 0xF
        ),
        0xE => match instruction[1] {
            0x9E => println!("{:<10} V{:X}", "SKIPKEY.Y.yellow()", instruction[0] & 0xF),
            0xA1 => println!("{:<10} V{:X}", "SKIPKEY.N.yellow()", instruction[0] & 0xF),
            _ => println!("{}", "UNKNOWN E".red()),
        },
        0xF => match instruction[1] {
            0x07 => println!("{:<10} V{:X}, DELAY", "MOV".yellow(), instruction[0] & 0xF),
            0x0A => println!("{:<10} V{:X}", "KEY".yellow(), instruction[0] & 0xF),
            0x15 => println!("{:<10} DELAY, V{:X}", "MOV".yellow(), instruction[0] & 0xF),
            0x18 => println!("{:<10} SOUND, V{:X}", "MOV".yellow(), instruction[0] & 0xF),
            0x1E => println!("{:<10} I, V{:X}", "ADI".yellow(), instruction[0] & 0xF),
            0x29 => println!(
                "{:<10} I, V{:X}",
                "SPRITECHAR".yellow(),
                instruction[0] & 0xF
            ),
            0x33 => println!("{:<10} (I), V{:X}", "MOVBCD".yellow(), instruction[0] & 0xF),
            0x55 => println!(
                "{:<10} (I), V0-V{:X}",
                "MOVM".yellow(),
                instruction[0] & 0xF
            ),
            0x65 => println!(
                "{:<10} V0-V{:X}, (I)",
                "MOVM".yellow(),
                instruction[0] & 0xF
            ),
            _ => println!("{}", "UNKNOWN F".red()),
        },
        _ => println!("{}", "UNKNOWN I".red()),
    }

    ()
}
