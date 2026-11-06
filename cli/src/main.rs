mod review;

use std::process::ExitCode;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "impa",
    version,
    about = "Reviews technical docs against the code in a lean, locked headless Claude"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run one review as its own headless claude session. Exits 1 if the review failed
    Review(review::Args),
}

fn main() -> ExitCode {
    let code = match Cli::parse().command {
        Command::Review(args) => review::run(args),
    };
    ExitCode::from(code)
}
