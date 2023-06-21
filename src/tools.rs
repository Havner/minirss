use std::{
    fs::File,
    io::{Read, Write}, net::TcpStream, time::Duration,
};

pub(crate) fn file_read(filename: &str) -> std::io::Result<Vec<u8>>
{
    let mut buf = Vec::<u8>::with_capacity(64);
    File::open(filename)?.read_to_end(&mut buf)?;
    buf.shrink_to_fit();
    println!("read file, len: {}", buf.len());
    Ok(buf)
}

#[allow(dead_code)]
pub(crate) fn file_write(filename: &str, data: &[u8]) -> std::io::Result<()>
{
    File::create(filename)?.write_all(data)
}

pub(crate) fn read_stream(stream: &mut TcpStream) -> std::io::Result<Option<Vec<u8>>>
{
    stream.set_read_timeout(None)?;

    let mut data = [0u8; 0x1000];
    let mut count = stream.read(&mut data)?;
    if count == 0 {
        return Ok(None);
    }

    /* ugly, but should work */
    stream.set_read_timeout(Some(Duration::from_millis(10)))?;
    loop {
        let mut buf = [0u8, 1];
        let result = stream.peek(&mut buf);
        if let Err(e) = &result {
            if e.kind() == std::io::ErrorKind::WouldBlock {
                break;
            }
        }
        let mut left = result.unwrap();
        if left > 0 {
            left = stream.read(&mut data[count..])?;
            count = count + left;
        } else {
            break;
        }
    }

    stream.set_read_timeout(None)?;

    Ok(Some(data[..count].to_vec()))
}
