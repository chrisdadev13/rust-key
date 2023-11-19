mod commands;

use commands::cli;

use clap::{arg, Command};

fn main() {
    let matches = cli().get_matches();
}
