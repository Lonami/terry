mod magic;
mod packet8;
mod player_buffs;
mod player_info;
mod player_mana;
mod player_uuid;
mod to_spawn;

use crate::serialization::{Deserializable, Serializable, SliceCursor};
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

    fn write_body(&self, cursor: &mut SliceCursor);

    fn from_body(&self, cursor: &mut SliceCursor) -> Self;
}

/*
impl Serializable for Packet {

    fn serialize(&self, cursor: &mut SliceCursor) {
        let length_pos = cursor.pos();
        cursor.write(0u16); // length
        cursor.write(0u8); // player
        cursor.write(Self::TAG);
        self.write_body(cursor);
        let length: u16 = (cursor.pos() - length_pos).try_into().expect("packet too long");
        cursor.rewrite(length_pos, length);
    }
}

impl Deserializable for Packet {

}
*/

pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl RGB {
    pub fn new() -> Self {
        RGB { r: 0, g: 0, b: 0 }
    }
}

impl Serializable for RGB {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.r);
        cursor.write(&self.g);
        cursor.write(&self.b);
    }
}

impl Deserializable for RGB {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        Self {
            r: cursor.read(),
            g: cursor.read(),
            b: cursor.read(),
        }
    }
}
