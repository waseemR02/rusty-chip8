use std::fs;

pub fn disasm(filepath: String) -> Result<(), Box<dyn std::error::Error>> {
    let buffer = fs::read(&filepath)?;
    let mut pc: u16 = 0x200;

    println!("Length: {}", buffer.len());
    println!("Disassembly of {}:\n", &filepath);
    for i in (0..buffer.len()).step_by(2) {
        println!(
            "  {pc:04X}:\t\t {:02X} {:02X}",
            buffer.get(i).unwrap(),
            buffer.get(i + 1).unwrap_or_else(|| &0x00),
        );
        pc += 0x02;
    }
    //println!("Buffer: {buffer:02X?}");
    Ok(())
}

//pub fn decode(code: &u16, pc: u16) -> &u16 {
//    let mut f_nibble = code & 0xF0;
//    f_nibble = f_nibble >> 12;
//    match f_nibble {
//        0x0 => {}
//        _ => {}
//    }
//
//    code
//}
