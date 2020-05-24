mod net;
mod packets;
mod serialization;

use std::env;
use std::io;

// TODO don't use a constant
const SERVER_ADDR: &str = "localhost:7777";

fn hex_digit_to_n(digit: u8) -> u8 {
    match digit {
        n @ b'0'..=b'9' => n - b'0',
        n @ b'a'..=b'f' => 10 + n - b'a',
        n @ b'A'..=b'F' => 10 + n - b'A',
        n => panic!(format!("'{}' is not a hex digit", n as char)),
    }
}

fn parse_hex(hex: &str) -> Vec<u8> {
    let mut result = Vec::with_capacity(hex.len() / 2);
    hex.as_bytes().chunks(2).for_each(|n| {
        result.push(hex_digit_to_n(n[0]) << 4 | hex_digit_to_n(n[1]));
    });
    result
}

fn main() -> io::Result<()> {
    if let Some(payload) = env::args().nth(1) {
        // TODO don't redo this here, reuse
        let mut payload = parse_hex(&payload);
        dbg!(packets::Packet::from_slice(&mut payload[2..]));
    } else {
        let _terraria = net::Terraria::connect(SERVER_ADDR)?;
    }

    Ok(())
}
