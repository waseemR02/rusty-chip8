use clap::{Parser, Subcommand};

mod dump;

#[derive(Parser)]
#[command(version, about, long_about=None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Disassembles the Chip8 ROM file
    Dump { filepath: String },
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Dump { filepath } => {
                dump::disasm(filepath)
                    .unwrap_or_else(|err| eprintln!("Error disassembling: {err}"));
            }
        }
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
