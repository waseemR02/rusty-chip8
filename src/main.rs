use std::process;

use clap::{Parser, Subcommand};
use minifb::{Window, WindowOptions};
use rusty_chip8::{chip::Chip, dump};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

#[derive(Subcommand)]
enum Command {
    Dump {
        #[arg(short, long)]
        filepath: String,
    },

    Emulate {
        #[arg(short, long)]
        filepath: String,
    },
}

#[derive(Parser)]
#[command(version, about, long_about=None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    let cli = Cli::parse();
    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    let mut window = Window::new("rusty-chip8", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    match &cli.command {
        Command::Dump { filepath } => {
            if let Err(e) = dump::disasm(filepath.clone()) {
                eprintln!("Error disassembling: {e}")
            }
        }
        Command::Emulate { filepath } => {
            let mut chip = Chip::new();
            if let Err(e) = chip.load(filepath.clone()) {
                eprintln!("Error loading the rom: {e}");
                process::exit(1);
            }
        }
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
