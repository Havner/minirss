mod rss;
mod stubs;
mod tcp;
mod tools;

use clap::Parser;

type GenericResult = Result<(), Box<dyn std::error::Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli
{
    /// filename with key, none for stub
    #[arg(short, long)]
    key: Option<String>,

    /// filename with token, none for stub
    #[arg(short, long)]
    token: Option<String>,
}

fn main() -> GenericResult
{
    let cli = Cli::parse();

    tcp::tcp(&cli)?;

    Ok(())
}
