use std::process;

use clap::Parser;
use rusty_chip8::chip::Chip;

#[derive(Parser)]
#[command(version, about, long_about=None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(long, short)]
    filepath: String,
}

fn main() {
    let cli = Cli::parse();

    let mut chip = Chip::new();
    if let Err(e) = chip.load(cli.filepath) {
        eprintln!("Error loading the rom: {e}");
        process::exit(1);
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
