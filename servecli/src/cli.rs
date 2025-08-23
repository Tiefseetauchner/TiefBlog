use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "tbserve",
    version,
    author = env!("CARGO_PKG_AUTHORS"),
    about = "The runner cli for tiefblog",
)]
pub(crate) struct Cli {
    #[arg(short, long, help = "Enable verbose output.", global = true)]
    pub verbose: bool,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    #[command(about = "Run server")]
    Run {},
}
