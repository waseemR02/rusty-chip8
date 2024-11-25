use std::fs;

pub fn disasm(filepath: String) -> Result<(), Box<dyn std::error::Error>> {
    let buffer = fs::read(filepath)?;
    println!("Buffer: {buffer:02X?}");
    Ok(())
}
