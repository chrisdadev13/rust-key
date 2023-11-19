mod cmd;
mod utils;

use clap::{Args, Parser, Subcommand};

use cmd::generate_password;
use dialoguer::{Input, Password};
use utils::clipboard_manager;

use key::*;

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
    /// Save one of your passwords/keys
    Save(SaveArgs),
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

#[derive(Args)]
struct SaveArgs {
    /// Set the name of the password account
    #[arg(short, long)]
    name: String,
}

fn main() {
    let cli = Cli::parse();

    let connection = &mut establish_connection();

    match &cli.command {
        Commands::Generate(args) => {
            let password =
                generate_password(args.length, args.numbers, args.symbols, args.uppercase);

            clipboard_manager(password)
        }
        Commands::Save(args) => {
            let url: String = Input::new()
                .with_prompt("Enter URL (optional)")
                .interact_text()
                .unwrap_or_default();
            let password: String = Password::new()
                .with_prompt("Enter Password")
                .interact()
                .unwrap();
            let category: String = Input::new()
                .with_prompt("Enter a Category (optional)")
                .interact_text()
                .unwrap_or_default();

            create_credentials(
                connection,
                Some(url.as_str()),
                args.name.as_str(),
                password.as_str(),
                Some(category.as_str()),
            )
        }
    }
}
