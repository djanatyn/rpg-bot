use serde::{Deserialize, Serialize};
use std::{io, str};
use tokio::net::TcpStream;
use tokio::process::Command;
use tokio::time::{sleep, Duration};

const LUA_SCRIPT: &str = "../final-fantasy.lua";
const ROM: &str = "../final-fantasy.zip";
const FCEUX_SOCKET: &str = "localhost:8080";
const FCEUX_START_WAIT: Duration = Duration::from_millis(1000);
const DELAY_BETWEEN_FRAMES: Duration = Duration::from_millis(16); // TODO: switch to interval?

#[derive(Debug, Deserialize)]
struct MemoryAddress {
    value: u64,
    tags: Vec<String>,
}

// https://fceux.com/web/help/LuaFunctionsList.html
// (joypad library -> joypad.write)
#[derive(Debug, Serialize, Clone, Copy)]
struct Input {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    #[serde(rename = "A")]
    a: bool,
    #[serde(rename = "B")]
    b: bool,
    start: bool,
    select: bool,
}

const NEUTRAL: Input = Input {
    up: false,
    down: false,
    left: false,
    right: false,
    a: false,
    b: false,
    start: false,
    select: false,
};

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

    let mut frame: u64 = 0;

    let stream: TcpStream = loop {
        // wait one second so fceux can launch
        sleep(FCEUX_START_WAIT).await;
        break match TcpStream::connect(FCEUX_SOCKET).await {
            Ok(success) => success,
            Err(_) => continue,
        };
    };

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

        let json = memory.trim().lines().collect::<Vec<_>>();
        let addresses: Vec<MemoryAddress> = serde_json::from_str(json[0]).expect("failed to parse");
        println!("{addresses:#?}");

        loop {
            stream.writable().await?;
            let num_addresses = addresses.len();
            // let output = format!("rust: got {num_addresses:?} addresses (frame {frame})\n");
            let game_input = Input {
                start: true,
                ..NEUTRAL
            };
            let input_json =
                serde_json::to_string(&game_input).expect("failed to convert input to JSON");
            let output = format!("{input_json}\n");

            match stream.try_write(&output.into_bytes()) {
                Ok(_) => break,
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(e) => return Err(e),
            }
        }

        sleep(DELAY_BETWEEN_FRAMES).await;
        frame += 1;
    }
}
