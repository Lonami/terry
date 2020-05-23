mod packets;

use crate::packets::{Packet, RGB};
use std::io::{self, Read, Write};
use std::net::TcpStream;

const PLAYER_NAME: &str = "terry";
const PLAYER_UUID: &str = "01032c81-623f-4435-85e5-e0ec816b09ca"; // random
const PROTOCOL_MAGIC: &str = "Terraria228";
const SERVER_ADDR: &str = "localhost:7777";

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect(SERVER_ADDR)?;
    let mut buffer = Vec::with_capacity(256);

    // TODO probably proxy should print incoming and outgoing
    //      messages as they occur so we do things properly,
    //      and not sure if uuid needs to be a proper value

    stream.write(
        packets::Magic {
            magic: PROTOCOL_MAGIC.to_string(),
        }
        .as_byte_slice(&mut buffer),
    )?;

    stream.write(
        packets::PlayerInfo {
            skin_variant: 0,
            hair_variant: 0,
            name: PLAYER_NAME.to_string(),
            hair_dye: 0,
            visible_accesories_flags: 0,
            hide_misc: false,
            hair_color: RGB::new(),
            skin_color: RGB::new(),
            eye_color: RGB::new(),
            shirt_color: RGB::new(),
            undershirt_color: RGB::new(),
            pants_color: RGB::new(),
            shoes_color: RGB::new(),
            difficulty_flags: 4,
        }
        .as_byte_slice(&mut buffer),
    )?;

    stream.write(
        packets::PlayerUuid {
            uuid4: PLAYER_UUID.to_string(),
        }
        .as_byte_slice(&mut buffer),
    )?;

    stream.write(
        packets::PlayerMana {
            mana: 200,
            max_mana: 200,
        }
        .as_byte_slice(&mut buffer),
    )?;

    stream.write(packets::PlayerBuffs { buffs: vec![] }.as_byte_slice(&mut buffer))?;

    // TODO figure out what this means, it's just a 6
    stream.write(&[3, 0, 6])?;

    stream.write(packets::Packet8 { n: -1 }.as_byte_slice(&mut buffer))?;

    stream.write(
        packets::ToSpawn {
            x: -1,
            y: -1,
            timer: 0,
            how: 1,
        }
        .as_byte_slice(&mut buffer),
    )?;

    // TODO figure out what this means
    stream.write(&[12, 0, 0x52, 0x06, 0x00, 0x0e, 0, 0, 0, 0, 0, 0x3f])?;

    // TODO figure out what this means
    stream.write(&[5, 0, 0x38, 0x01, 0])?;

    stream.flush()?;

    let mut buffer = vec![0; 1024];
    while let Ok(n) = stream.read(&mut buffer) {
        eprintln!("{:?}", &buffer[..n]);
    }

    Ok(())
}
