use terraria_protocol;

use std::io;

// TODO don't use a constant
const SERVER_ADDR: &str = "localhost:7777";

fn main() -> io::Result<()> {
    println!("Connecting to the server...");
    let _terraria = terraria_protocol::net::Terraria::connect(SERVER_ADDR)?;
    println!("Connected to the server!");
    Ok(())
}
