use clap::Command;

pub fn cli() -> Command {
    Command::new("key")
        .version("1.0")
        .author("Your Name")
        .about("🔒 A simple password/keys manager CLI")
        .subcommand(Command::new("generate").about("Generate a password"))
        .subcommand(Command::new("save").about("Save a password"))
        .subcommand(Command::new("retrieve").about("Retrieve a password"))
        .subcommand(Command::new("update").about("Update a password"))
}
