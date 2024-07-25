mod echo;

use clap::{Parser, Subcommand};
use crate::echo::Echo;

#[derive(Parser)]
#[command(
    version = "1.0.0",
    about = "A bunch of common CLI tools re-implemented in Rust",
    long_about = None,
    propagate_version = true
)]
struct RustCli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Displays the entered message after re-formatting it
    Echo(Echo)

    // Add new commands here
}

fn main() {
    let command_line = RustCli::parse();

    match command_line.command {
        Commands::Echo(echo) => { echo::parse(echo) }
        // Parse new commands here
    }
}
