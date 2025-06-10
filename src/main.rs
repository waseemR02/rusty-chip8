use std::process;

use clap::{Parser, Subcommand};
use minifb::{Key, KeyRepeat, Scale, ScaleMode, Window, WindowOptions};
use rusty_chip8::{chip::Chip, dump, instructions::Instruction};

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

    match &cli.command {
        Command::Dump { filepath } => {
            if let Err(e) = dump::disasm(filepath.clone()) {
                eprintln!("Error disassembling: {e}")
            }
        }
        Command::Emulate { filepath } => {
            let mut buffer = vec![0u32; WIDTH * HEIGHT];

            let mut window = Window::new(
                "rusty-chip8",
                WIDTH,
                HEIGHT,
                WindowOptions {
                    resize: true,
                    scale: Scale::X16,
                    scale_mode: ScaleMode::AspectRatioStretch,
                    ..WindowOptions::default()
                },
            )
            .unwrap();
            window.set_target_fps(250);
            let mut chip = Chip::new();
            if let Err(e) = chip.load(filepath.clone()) {
                eprintln!("Error loading the rom: {e}");
                process::exit(1);
            }

            while window.is_open() && !window.is_key_down(Key::Escape) {
                // if window.is_key_pressed(Key::J, KeyRepeat::No) {
                println!("{}", chip);
                chip.interpret(
                    Instruction::new(&[chip.mem[chip.pc as usize], chip.mem[chip.pc as usize + 1]]),
                    &mut buffer,
                    &window,
                );
                // }
                window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
            }
        }
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
