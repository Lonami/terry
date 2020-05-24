mod magic;
mod packet8;
mod packet82;
mod player_buffs;
mod player_info;
mod player_mana;
mod player_uuid;
mod to_spawn;

use crate::serialization::{Deserializable, Serializable, SliceCursor};
pub use magic::Magic;
pub use packet8::Packet8;
pub use packet82::Packet82;
pub use player_buffs::PlayerBuffs;
pub use player_info::PlayerInfo;
pub use player_mana::PlayerMana;
pub use player_uuid::PlayerUuid;
use std::convert::TryInto;
pub use to_spawn::ToSpawn;

pub trait PacketBody: Sized {
    const TAG: u8;

    fn write_body(&self, cursor: &mut SliceCursor);

    fn from_body(cursor: &mut SliceCursor) -> Self;

    // TODO player should probably go inside the packets
    fn serialize(&self, player: Option<u8>, cursor: &mut SliceCursor) {
        let length_pos = cursor.pos();
        cursor.write(&0u16); // length
        if let Some(player) = player {
            cursor.write(&player); // player
        }
        cursor.write(&Self::TAG);
        self.write_body(cursor);
        let length: u16 = (cursor.pos() - length_pos)
            .try_into()
            .expect("packet too long");
        cursor.rewrite(length_pos, &length);
    }
}

#[derive(Debug)]
pub enum Packet {
    Magic(Magic),
    Packet8(Packet8),
    PlayerInfo(PlayerInfo),
    ToSpawn(ToSpawn),
    PlayerMana(PlayerMana),
    PlayerBuffs(PlayerBuffs),
    PlayerUuid(PlayerUuid),
    Packet82(Packet82),
}

impl Packet {
    pub fn from_slice(slice: &mut [u8]) -> (u8, Self) {
        let mut cursor = SliceCursor::new(slice);
        let tag = cursor.read::<u8>();
        let player = cursor.read::<u8>();
        // TODO too bad packet body is not serializable
        (
            player,
            match tag {
                Magic::TAG => Self::Magic(Magic::from_body(&mut cursor)),
                Packet8::TAG => Self::Packet8(Packet8::from_body(&mut cursor)),
                PlayerInfo::TAG => Self::PlayerInfo(PlayerInfo::from_body(&mut cursor)),
                ToSpawn::TAG => Self::ToSpawn(ToSpawn::from_body(&mut cursor)),
                PlayerMana::TAG => Self::PlayerMana(PlayerMana::from_body(&mut cursor)),
                PlayerBuffs::TAG => Self::PlayerBuffs(PlayerBuffs::from_body(&mut cursor)),
                PlayerUuid::TAG => Self::PlayerUuid(PlayerUuid::from_body(&mut cursor)),
                Packet82::TAG => Self::Packet82(Packet82::from_body(&mut cursor)),
                tag => panic!(format!("unknown tag {}", tag)),
            },
        )
    }
}

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_good_packet_serialization() {
        let mut buffer = vec![0; 64];
        let mut cursor = SliceCursor::new(buffer.as_mut_slice());
        Magic { magic: "Terraria228".to_string() }.serialize(None, &mut cursor);
        let pos = cursor.finish();
        assert_eq!(&buffer[..pos], &[
            0x0f, 0x00, 0x01, 0x0b, 0x54, 0x65, 0x72, 0x72, 0x61, 0x72, 0x69, 0x61, 0x32, 0x32, 0x38
        ]);
    }
}
