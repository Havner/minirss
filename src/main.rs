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
    /// Key response generator
    Key(subcmds::KeyArgs),
    /// Token response generator
    Token(subcmds::TokenArgs),
    /// automatic TCP mode
    Tcp(subcmds::TcpArgs),
}

fn main() -> GenericResult
{
    let cli = Cli::parse();

    match &cli.command {
        Commands::Key(args) => subcmds::key(args)?,
        Commands::Token(args) => subcmds::token(args)?,
        Commands::Tcp(args) => subcmds::tcp(args)?,
    };

    Ok(())
}
