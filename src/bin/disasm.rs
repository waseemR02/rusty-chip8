use clap::{Parser, Subcommand};
use rusty_chip8::dump;

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
            todo!()
        }
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
