use std::io::prelude::*;
use std::net::TcpStream;
use std::str;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8000")?;

    let tempstr = "test1";
    let mut buf = [0; 10];

    stream.write(tempstr.as_bytes())?;
    stream.read(& mut buf)?;
    print!("{}", str::from_utf8(&buf).unwrap());
    Ok(())
} 
