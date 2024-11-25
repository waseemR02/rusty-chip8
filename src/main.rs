use std::fs;

fn main() {
    let buffer = fs::read("roms/IBM_Logo.ch8");
    println!("Buffer: {buffer:02X?}");
}
