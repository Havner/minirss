mod rss;
mod stubs;
mod subcmds;
mod tools;

use clap::{Parser, Subcommand};

type GenericResult = Result<(), Box<dyn std::error::Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli
{
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands
{
    /// automatic TCP mode
    Tcp(subcmds::TcpArgs),
}

fn main() -> GenericResult
{
    let cli = Cli::parse();

    match &cli.command {
        Commands::Tcp(args) => subcmds::tcp(args)?,
    };

    Ok(())
}
