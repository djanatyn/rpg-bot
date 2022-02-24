//! Sofia (like the Dragon Quest IV Hero)!
//!
//! An experiment controlling an NES emulator using Rust.
//! FCEUX lua APIs are used to read memory addresses and control input.
use serde::{Deserialize, Serialize};
use std::{io, str};
use tokio::net::TcpStream;
use tokio::process::Command;
use tokio::time::{sleep, Duration};

/// Path of FCEUX lua script, passed as argument to --loadlua.
const LUA_SCRIPT: &str = "../final-fantasy.lua";
/// Path of Final Fantasy 1 NES ROM.
const ROM: &str = "../final-fantasy.zip";
/// Hostname and port of FCEUX listening TCP socket.
const FCEUX_SOCKET: &str = "localhost:8080";
/// Time to wait between checking for FCEUX TCP socket.
const FCEUX_START_WAIT: Duration = Duration::from_millis(1000);
/// Time to wait between frames.
const DELAY_BETWEEN_FRAMES: Duration = Duration::from_millis(16); // TODO: switch to interval?

/// Value of a memory address read from FCEUX.
///
/// Received over network during emulation. Each address has a set of tags.
/// Each address may be used for different purposes, in different situations
/// during runtime.
#[derive(Debug, Deserialize)]
struct MemoryAddress {
    value: u64,
    tags: Vec<String>,
}

/// Virtual controller input.
///
/// Serialized into JSON to send to FCEUX.
/// - <https://fceux.com/web/help/LuaFunctionsList.html>
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

/// Neutral input on the virtual controller (no buttons pressed).
///
/// Useful with struct update syntax:
/// ```
/// let game_input = Input { start: true, ..NEUTRAL };
/// assert_eq(game_input, Input {
///     up: false,
///     down: false,
///     left: false,
///     right: false,
///     a: false,
///     b: false,
///     start: true,
///     select: false
/// });
/// ```
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

/// Launch FCEUX and connect to lua socket. Read values of memory addresses and
/// send serialized inputs in a loop.
#[tokio::main]
async fn main() -> io::Result<()> {
    // spawn fceux, with listening lua server
    let mut fceux = Command::new("fceux")
        .arg("--loadlua")
        .arg(LUA_SCRIPT)
        .arg(ROM)
        .spawn()
        .expect("failed to spawn fceux");

    // signal handler, child fceux process needs SIGKILL
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        println!("sending kill signal to fceux");
        fceux.kill().await.expect("kill failed");
    });

    // attempt to connect to fceux lua server
    let stream: TcpStream = loop {
        // wait one second so fceux can launch
        sleep(FCEUX_START_WAIT).await;
        break match TcpStream::connect(FCEUX_SOCKET).await {
            Ok(success) => success,
            Err(_) => continue,
        };
    };

    let mut frame: u64 = 0;
    loop {
        let mut buf = [0; 4096];
        let mut memory = String::new();

        // read input from socket
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

        // parse live memory address values
        let json = memory.trim().lines().collect::<Vec<_>>();
        let addresses: Vec<MemoryAddress> = serde_json::from_str(json[0]).expect("failed to parse");
        println!("{addresses:#?}");

        let num_addresses = addresses.len();
        println!("rust: got {num_addresses:?} addresses (frame {frame})\n");

        // press START on virtual controller
        // TODO: dynamic input based on memory values (state machine)
        let game_input = Input {
            start: true,
            ..NEUTRAL
        };

        // convert controller input to json
        let input_json =
            serde_json::to_string(&game_input).expect("failed to convert input to JSON");

        // send controller input over socket
        loop {
            stream.writable().await?;
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
