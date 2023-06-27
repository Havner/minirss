use crate::{tools, rss, stubs, GenericResult};
use std::{net::TcpStream, io::Write, time::Duration};

pub(crate) fn tcp(args: &super::Cli) -> GenericResult
{
    println!("   === Preparing data ===\n");

    print!("KEY: ");
    let key = match args.key.as_ref() {
        Some(file) => tools::file_read(&file)?,
        None => {
            println!("taking stubbed");
            stubs::DELEGATED_KEY.to_vec()
        },
    };

    print!("TOKEN: ");
    let token = match args.token.as_ref() {
        Some(file) => tools::file_read(&file)?,
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

        // just to stabilize the channel, no flow control, the serial on FVP
        // can be unrealiable and to give time to handle the response
        std::thread::sleep(Duration::from_millis(100));
    }
}
