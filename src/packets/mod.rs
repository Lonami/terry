mod magic;
mod packet8;
mod player_buffs;
mod player_info;
mod player_mana;
mod player_uuid;
mod to_spawn;

pub use magic::Magic;
pub use packet8::Packet8;
pub use player_buffs::PlayerBuffs;
pub use player_info::PlayerInfo;
pub use player_mana::PlayerMana;
pub use player_uuid::PlayerUuid;
use std::convert::TryInto;
pub use to_spawn::ToSpawn;

pub trait Packet {
    const TAG: u8;

    fn append_body(&self, buf: &mut Vec<u8>);

    /// Clears the buffer and returns the slice it wrote to.
    /// Intended for easy use when writing to a stream.
    fn as_byte_slice<'a>(&self, buf: &'a mut Vec<u8>) -> &'a [u8] {
        buf.clear();
        buf.extend(&[0, 0, 0]); // length, length, player
        buf.push(Self::TAG);
        self.append_body(buf);
        let len: u16 = buf.len().try_into().expect("packet too long");
        buf[0..2].copy_from_slice(&len.to_le_bytes());
        &buf[..]
    }
}

pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl RGB {
    pub fn new() -> Self {
        RGB { r: 0, g: 0, b: 0 }
    }

    fn append_body(&self, buf: &mut Vec<u8>) {
        buf.push(self.r);
        buf.push(self.g);
        buf.push(self.b);
    }
}
