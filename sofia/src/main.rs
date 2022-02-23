use std::{io, str};
use tokio::net::TcpStream;
use tokio::process::Command;
use tokio::time::{sleep, Duration};

const LUA_SCRIPT: &str = "../final-fantasy.lua";
const ROM: &str = "../final-fantasy.zip";
const FCEUX_SOCKET: &str = "localhost:8080";

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut fceux = Command::new("fceux")
        .arg("--loadlua")
        .arg(LUA_SCRIPT)
        .arg(ROM)
        .spawn()
        .expect("failed to spawn fceux");

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        println!("sending kill signal to fceux");
        fceux.kill().await.expect("kill failed");
    });

    // wait one second so fceux can launch
    // TODO: poll for socket instead
    sleep(Duration::from_millis(1000)).await;

    let stream = TcpStream::connect(FCEUX_SOCKET).await?;

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
