mod clipboard_manager;
mod generate_password;

use clap::{Args, Parser, Subcommand};
use clipboard_manager::clipboard_manager;
use generate_password::generate_password;

#[derive(Parser)]
#[command(author="Chris P. <chrisdadev13@gmail.com>", version = "0.1.0", about = "🔒 A simple password/keys manager CLI", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a safe password/key
    Generate(GenerateArgs),
}

#[derive(Args)]
struct GenerateArgs {
    /// Generate a safe password/key
    #[arg(short, long)]
    length: u32,
    #[arg(short, long)]
    numbers: bool,
    #[arg(short, long)]
    symbols: bool,
    #[arg(short, long)]
    uppercase: bool,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate(args) => {
            let password =
                generate_password(args.length, args.numbers, args.symbols, args.uppercase);

            clipboard_manager(password)
        }
    }
}
