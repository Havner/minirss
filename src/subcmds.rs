use crate::{tools, rss, stubs, GenericResult};
use clap::Args;
use std::{net::TcpStream, io::Write};

#[derive(Args, Debug)]
pub(crate) struct KeyArgs
{
    /// filename with msg
    #[arg(short, long)]
    input: String,

    /// filename with key, none for stub
    #[arg(short, long)]
    key: Option<String>,

    /// filename to write to, none for stdout
    #[arg(short, long)]
    output: Option<String>,
}

pub(crate) fn key(args: &KeyArgs) -> GenericResult
{
    let input = tools::file_read(&args.input, true)?;
    let key = match args.key.as_ref() {
        Some(file) => tools::file_read(&file, false)?,
        None => stubs::DELEGATED_KEY.to_vec(),
    };
    let output = rss::do_key(&input, &key)?;

    if let Some(file) = args.output.as_ref() {
        tools::file_write(file, &output)?;
    }

    Ok(())
}

#[derive(Args, Debug)]
pub(crate) struct TokenArgs
{
    /// filename with msg
    #[arg(short, long)]
    input: String,

    /// filename with token, none for stub
    #[arg(short, long)]
    token: Option<String>,

    /// filename to write to, none for stdout
    #[arg(short, long)]
    output: Option<String>,
}

pub(crate) fn token(args: &TokenArgs) -> GenericResult
{
    let input = tools::file_read(&args.input, true)?;
    let token = match args.token.as_ref() {
        Some(file) => tools::file_read(&file, false)?,
        None => stubs::PLATFORM_TOKEN.to_vec(),
    };
    let output = rss::do_token(&input, &token)?;

    if let Some(file) = args.output.as_ref() {
        tools::file_write(file, &output)?;
    }

    Ok(())
}

#[derive(Args, Debug)]
pub(crate) struct TcpArgs
{
    /// filename with key, none for stub
    #[arg(short, long)]
    key: Option<String>,

    /// filename with token, none for stub
    #[arg(short, long)]
    token: Option<String>,
}

pub(crate) fn tcp(args: &TcpArgs) -> GenericResult
{
    println!("   === Preparing data ===\n");

    print!("KEY: ");
    let key = match args.key.as_ref() {
        Some(file) => tools::file_read(&file, false)?,
        None => {
            println!("taking stubbed");
            stubs::DELEGATED_KEY.to_vec()
        },
    };

    print!("TOKEN: ");
    let token = match args.token.as_ref() {
        Some(file) => tools::file_read(&file, false)?,
        None => {
            println!("taking stubbed");
            stubs::PLATFORM_TOKEN.to_vec()
        },
    };
    println!("");

    println!("   === Connecting to FVP ===\n");
    let mut stream = TcpStream::connect("localhost:5002")?;

    println!("   === Running loop ===\n");
    loop {
        let data = tools::read_stream(&mut stream)?;
        if let None = data {
            println!("   === Disconnected ===");
            break Ok(())
        }

        let req = data.unwrap();
        println!("TCP received request, len: {}", req.len());

        let res = rss::do_request(&req, &key, &token)?;

        println!("TCP sending response, len: {}\n", res.len());
        stream.write_all(&res)?;
    }
}
