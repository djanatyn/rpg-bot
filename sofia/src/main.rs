use std::{io, str};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let stream = TcpStream::connect("localhost:8080").await?;

    loop {
        let mut buf = [0; 4096];
        let mut memory = String::new();

        loop {
            stream.readable().await?;
            match stream.try_read(&mut buf) {
                Ok(0) => break,
                Ok(_) => {
                    memory = str::from_utf8(&buf).expect("couldn't parse").to_string();
                    break;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(e) => return Err(e),
            }
        }

        loop {
            stream.writable().await?;
            let output = format!("{memory}\n");
            match stream.try_write(&output.into_bytes()) {
                Ok(_) => break,
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(e) => return Err(e),
            }
        }
    }
}
