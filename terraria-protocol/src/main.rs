use std::io::{self, BufRead};
use terraria_protocol::Packet;

fn parse_hex(hex: &str) -> Vec<u8> {
    let mut result = Vec::with_capacity(hex.len() / 2);
    (0..hex.len()).step_by(2).for_each(|i| {
        result.push(u8::from_str_radix(&hex[i..i + 2], 16).expect("bad hex"));
    });
    result
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let mut payload = parse_hex(&line.unwrap());
        dbg!(Packet::from_slice(&mut payload[2..]));
    }
}
