mod packets;

use crate::packets::{Packet, RGB};
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

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

    eprintln!("sending magic");
    stream.write_all(
        packets::Magic {
            magic: PROTOCOL_MAGIC.to_string(),
        }
        .as_byte_slice(&mut buffer),
    )?;
    thread::sleep(Duration::from_secs(5));

    eprintln!("sending player info");
    stream.write_all(
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
    thread::sleep(Duration::from_secs(5));

    eprintln!("sending player uuid");
    stream.write_all(
        packets::PlayerUuid {
            uuid4: PLAYER_UUID.to_string(),
        }
        .as_byte_slice(&mut buffer),
    )?;
    thread::sleep(Duration::from_secs(5));

    eprintln!("sending player mana");
    stream.write_all(
        packets::PlayerMana {
            mana: 200,
            max_mana: 200,
        }
        .as_byte_slice(&mut buffer),
    )?;
    thread::sleep(Duration::from_secs(5));

    eprintln!("sending player buffs");
    stream.write_all(packets::PlayerBuffs { buffs: vec![] }.as_byte_slice(&mut buffer))?;
    thread::sleep(Duration::from_secs(5));

    eprintln!("sending 6");
    // TODO figure out what this means, it's just a 6
    stream.write_all(&[3, 0, 6])?;
    thread::sleep(Duration::from_secs(5));

    eprintln!("sending 8");
    stream.write_all(packets::Packet8 { n: -1 }.as_byte_slice(&mut buffer))?;
    thread::sleep(Duration::from_secs(5));

    eprintln!("sending to spawn");
    stream.write_all(
        packets::ToSpawn {
            x: -1,
            y: -1,
            timer: 0,
            how: 1,
        }
        .as_byte_slice(&mut buffer),
    )?;
    thread::sleep(Duration::from_secs(5));

    // 127.0.0.1:41124 was booted: Invalid operation at this state.

    eprintln!("sending to 0x52");
    // TODO figure out what this means
    stream.write_all(&[12, 0, 0x52, 0x06, 0x00, 0x0e, 0, 0, 0, 0, 0, 0x3f])?;
    thread::sleep(Duration::from_secs(5));

    eprintln!("sending to 0x38");
    // TODO figure out what this means
    stream.write_all(&[5, 0, 0x38, 0x01, 0])?;
    thread::sleep(Duration::from_secs(5));

    stream.flush()?;

    let mut buffer = vec![0; 1024];
    while let Ok(n) = stream.read(&mut buffer) {
        if n == 0 {
            break;
        }
        eprintln!("{:?}", &buffer[..n]);
    }

    Ok(())
}
