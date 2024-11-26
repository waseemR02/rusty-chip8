use std::fs;

pub fn disasm(filepath: String) -> Result<(), Box<dyn std::error::Error>> {
    let buffer = fs::read(&filepath)?;
    let mut pc: u16 = 0x200;

    println!("Disassembly of {}:\n", &filepath);
    for i in (0..buffer.len()).step_by(2) {
        let instruction = [
            buffer.get(i).unwrap(),
            buffer.get(i + 1).unwrap_or_else(|| &0x00),
        ];
        decode(instruction, pc);
        pc += 0x02;
    }
    Ok(())
}

pub fn decode(instruction: [&u8; 2], pc: u16) -> () {
    print!(
        "  {pc:04X}:\t\t {:02X} {:02X}\t",
        instruction[0], instruction[1]
    );

    let nibble = instruction[0] >> 4;
    //println!("\t\t{nibble:02X}");
    match nibble {
        0x0 => {
            if *instruction[0] == 0x00 {
                match instruction[1] {
                    0xE0 => println!("{:<10}", "CLS"),
                    0xEE => println!("{:<10}", "RTS"),
                    _ => println!("UNKNOWN 0"),
                }
            } else {
                println!("UNKNOWN 0")
            }
        }
        0x1 => println!(
            "{:<10} ${:X}{:02X}",
            "JUMP",
            instruction[0] & 0xF,
            instruction[1]
        ),
        0x2 => println!(
            "{:<10} ${:X}{:02X}",
            "CALL",
            instruction[0] & 0xF,
            instruction[1]
        ),
        0x3 => println!(
            "{:<10} V{:X}, #${:02X}",
            "SKIP.EQ",
            instruction[0] & 0xF,
            instruction[1]
        ),
        0x4 => println!(
            "{:<10} V{:X}, #${:02X}",
            "SKIP.NE",
            instruction[0] & 0xF,
            instruction[1]
        ),
        0x5 => println!(
            "{:<10} V{:X}, V{:X}",
            "SKIP.EQ",
            instruction[0] & 0xF,
            (instruction[1] & 0xF0) >> 4
        ),
        0x6 => println!(
            "{:<10} V{:X}, #${:02X}",
            "MVI",
            instruction[0] & 0xF,
            instruction[1]
        ),
        0x7 => println!(
            "{:<10} V{:X}, #${:02X}",
            "ADI",
            instruction[0] & 0xF,
            instruction[1]
        ),
        0x8 => match instruction[1] & 0xF {
            0x0 => println!(
                "{:<10} V{:X}, V{:X}",
                "MOV",
                instruction[0] & 0xF,
                instruction[1] >> 4,
            ),
            0x1 => println!(
                "{:<10} V{:X}, V{:X}",
                "OR",
                instruction[0] & 0xF,
                instruction[1] >> 4
            ),
            0x2 => println!(
                "{:<10} V{:X}, V{:X}",
                "AND",
                instruction[0] & 0xF,
                instruction[1] >> 4
            ),
            0x3 => println!(
                "{:<10} V{:X}, V{:X}",
                "XOR",
                instruction[0] & 0xF,
                instruction[1] >> 4
            ),
            0x4 => println!(
                "{:<10} V{:X}, V{:X}",
                "ADD.",
                instruction[0] & 0xF,
                instruction[1] >> 4
            ),
            0x5 => println!(
                "{:<10} V{:X}, V{:X}",
                "SUB.",
                instruction[0] & 0xF,
                instruction[1] >> 4
            ),
            0x6 => println!("{:<10} V{:X}", "SHR.", instruction[0] & 0xF,),
            0x7 => println!(
                "{:<10} V{:X}, V{:X}",
                "SUBN.",
                instruction[0] & 0xF,
                instruction[1] >> 4
            ),
            0xE => println!("{:<10} V{:X}", "SHL.", instruction[0] & 0xF,),
            _ => println!("UNKNOWN 8"),
        },
        0x9 => println!(
            "{:<10} V{:X}, V{:X}",
            "SKIP.NE",
            instruction[0] & 0xF,
            instruction[1] >> 4
        ),
        0xA => println!(
            "{:<10} I, #${:X}{:02X}",
            "MVI",
            instruction[0] & 0xF,
            instruction[1]
        ),
        0xB => println!(
            "{:<10} #${:X}{:02X}(V0)",
            "JUMP",
            instruction[0] & 0xF,
            instruction[1]
        ),
        0xC => println!(
            "{:<10} V{:X}, #${:02X}",
            "RNDMSK",
            instruction[0] & 0xF,
            instruction[1]
        ),
        0xD => println!(
            "{:<10} V{:X}, V{:X}, #${:X}",
            "DRAW",
            instruction[0] & 0xF,
            instruction[1] >> 4,
            instruction[1] & 0xF
        ),
        0xE => match instruction[1] {
            0x9E => println!("{:<10} V{:X}", "SKIPKEY.Y", instruction[0] & 0xF),
            0xA1 => println!("{:<10} V{:X}", "SKIPKEY.N", instruction[0] & 0xF),
            _ => println!("UNKNOWN E"),
        },
        0xF => match instruction[1] {
            0x07 => println!("{:<10} V{:X}, DELAY", "MOV", instruction[0] & 0xF),
            0x0A => println!("{:<10} V{:X}", "KEY", instruction[0] & 0xF),
            0x15 => println!("{:<10} DELAY, V{:X}", "MOV", instruction[0] & 0xF),
            0x18 => println!("{:<10} SOUND, V{:X}", "MOV", instruction[0] & 0xF),
            0x1E => println!("{:<10} I, V{:X}", "ADI", instruction[0] & 0xF),
            0x29 => println!("{:<10} I, V{:X}", "SPRITECHAR", instruction[0] & 0xF),
            0x33 => println!("{:<10} (I), V{:X}", "MOVBCD", instruction[0] & 0xF),
            0x55 => println!("{:<10} (I), V0-V{:X}", "MOVM", instruction[0] & 0xF),
            0x65 => println!("{:<10} V0-V{:X}, (I)", "MOVM", instruction[0] & 0xF),
            _ => println!("UNKNOWN F"),
        },
        _ => println!("UNKNOWN I"),
    }

    ()
}
