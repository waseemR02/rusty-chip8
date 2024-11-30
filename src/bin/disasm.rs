use clap::Parser;
use rusty_chip8::dump;

#[derive(Parser)]
#[command(version, about, long_about=None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(long, short)]
    filepath: String,
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = dump::disasm(cli.filepath) {
        eprintln!("Error disassembling: {e}")
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
