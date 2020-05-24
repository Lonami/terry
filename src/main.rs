mod net;
mod packets;
mod serialization;

use std::io;

// TODO don't use a constant
const SERVER_ADDR: &str = "localhost:7777";

fn main() -> io::Result<()> {
    let _terraria = net::Terraria::connect(SERVER_ADDR)?;
    Ok(())
}
