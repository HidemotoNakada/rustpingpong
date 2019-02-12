use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};



fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; 10];

    let _len = stream.read(& mut buf[..])?;
    stream.write(& mut buf)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?)?;
    }
    Ok(())
}
