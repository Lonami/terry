use terraria_protocol::net::Terraria;
use terraria_protocol::packets::{self, Packet};
use std::thread;
use std::time::Duration;

use std::io;

// TODO don't use a constant
const SERVER_ADDR: &str = "localhost:7777";

fn main() -> io::Result<()> {
    println!("Connecting to the server...");
    let mut terraria = Terraria::connect(SERVER_ADDR)?;
    println!("Connected to the server!");

    let info = loop {
        match terraria.recv_packet() {
            Ok(Packet::WorldInfo(info)) => break info,
            Ok(_) => continue,
            Err(_) => {
                println!("Could not receive world info :-(");
                return Ok(());
            }
        }
    };

    terraria.send_packet(&packets::LoadNetModule::ClientText {
        command: "Say".to_string(),
        text: format!(
            "Hi, I'm terry and I sure love to spawn at ({}, {})!",
            info.spawn_x, info.spawn_y
        ),
    })?;
    thread::sleep(Duration::from_secs(4));
    terraria.send_packet(&packets::LoadNetModule::ClientText {
        command: "Say".to_string(),
        text: format!(
            "I love how {} is {}x{} big, so much space!!",
            info.world_name, info.max_tiles_x, info.max_tiles_y,
        ),
    })?;
    thread::sleep(Duration::from_secs(4));

    Ok(())
}
