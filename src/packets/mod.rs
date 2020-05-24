mod compressed_tile_block;
mod item_moved;
mod kill_count;
mod magic;
mod npc_info;
mod packet11;
mod packet22;
mod packet3;
mod packet49;
mod packet57;
mod packet6;
mod packet7;
mod packet8;
mod packet82;
mod packet9;
mod player_buffs;
mod player_info;
mod player_inventory;
mod player_life;
mod player_mana;
mod player_move;
mod player_uuid;
mod to_spawn;

use crate::serialization::{Deserializable, Serializable, SliceCursor};
pub use compressed_tile_block::CompressedTileBlock;
pub use item_moved::ItemMoved;
pub use kill_count::KillCount;
pub use magic::Magic;
pub use npc_info::NpcInfo;
pub use packet11::Packet11;
pub use packet22::Packet22;
pub use packet3::Packet3;
pub use packet49::Packet49;
pub use packet57::Packet57;
pub use packet6::Packet6;
pub use packet7::Packet7;
pub use packet8::Packet8;
pub use packet82::Packet82;
pub use packet9::Packet9;
pub use player_buffs::PlayerBuffs;
pub use player_info::PlayerInfo;
pub use player_inventory::PlayerInventory;
pub use player_life::PlayerLife;
pub use player_mana::PlayerMana;
pub use player_move::PlayerMove;
pub use player_uuid::PlayerUuid;
use std::convert::TryInto;
pub use to_spawn::ToSpawn;

pub trait PacketBody: Sized {
    const TAG: u8;

    fn write_body(&self, cursor: &mut SliceCursor);

    fn from_body(cursor: &mut SliceCursor) -> Self;

    // TODO player should probably go inside the packets
    fn serialize(&self, cursor: &mut SliceCursor) {
        let length_pos = cursor.pos();
        cursor.write(&0u16); // length
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
    Magic(Magic),                             // 1
    Packet3(Packet3),                         // 3
    PlayerInfo(PlayerInfo),                   // 4
    PlayerInventory(PlayerInventory),         // 5
    Packet6(Packet6),                         // 6
    Packet7(Packet7),                         // 7
    Packet8(Packet8),                         // 8
    Packet9(Packet9),                         // 9
    CompressedTileBlock(CompressedTileBlock), // 10
    Packet11(Packet11),                       // 11
    ToSpawn(ToSpawn),                         // 12
    PlayerMove(PlayerMove),                   // 13
    PlayerLife(PlayerLife),                   // 16
    ItemMoved(ItemMoved),                     // 21
    Packet22(Packet22),                       // 22
    NpcInfo(NpcInfo),                         // 23
    PlayerMana(PlayerMana),                   // 42
    Packet49(Packet49),                       // 42
    PlayerBuffs(PlayerBuffs),                 // 50
    Packet57(Packet57),                       // 57
    PlayerUuid(PlayerUuid),                   // 68
    Packet82(Packet82),                       // 82
    KillCount(KillCount),                     // 83
}

impl Packet {
    pub fn from_slice(slice: &mut [u8]) -> Self {
        let mut cursor = SliceCursor::new(slice);
        let tag = cursor.read::<u8>();
        // TODO too bad packet body is not serializable
        match tag {
            Magic::TAG => Self::Magic(Magic::from_body(&mut cursor)),
            Packet3::TAG => Self::Packet3(Packet3::from_body(&mut cursor)),
            PlayerInfo::TAG => Self::PlayerInfo(PlayerInfo::from_body(&mut cursor)),
            PlayerInventory::TAG => Self::PlayerInventory(PlayerInventory::from_body(&mut cursor)),
            Packet6::TAG => Self::Packet6(Packet6::from_body(&mut cursor)),
            Packet7::TAG => Self::Packet7(Packet7::from_body(&mut cursor)),
            Packet8::TAG => Self::Packet8(Packet8::from_body(&mut cursor)),
            Packet9::TAG => Self::Packet9(Packet9::from_body(&mut cursor)),
            CompressedTileBlock::TAG => {
                Self::CompressedTileBlock(CompressedTileBlock::from_body(&mut cursor))
            }
            Packet11::TAG => Self::Packet11(Packet11::from_body(&mut cursor)),
            ToSpawn::TAG => Self::ToSpawn(ToSpawn::from_body(&mut cursor)),
            PlayerMove::TAG => Self::PlayerMove(PlayerMove::from_body(&mut cursor)),
            PlayerLife::TAG => Self::PlayerLife(PlayerLife::from_body(&mut cursor)),
            ItemMoved::TAG => Self::ItemMoved(ItemMoved::from_body(&mut cursor)),
            Packet22::TAG => Self::Packet22(Packet22::from_body(&mut cursor)),
            NpcInfo::TAG => Self::NpcInfo(NpcInfo::from_body(&mut cursor)),
            PlayerMana::TAG => Self::PlayerMana(PlayerMana::from_body(&mut cursor)),
            Packet49::TAG => Self::Packet49(Packet49::from_body(&mut cursor)),
            PlayerBuffs::TAG => Self::PlayerBuffs(PlayerBuffs::from_body(&mut cursor)),
            Packet57::TAG => Self::Packet57(Packet57::from_body(&mut cursor)),
            PlayerUuid::TAG => Self::PlayerUuid(PlayerUuid::from_body(&mut cursor)),
            Packet82::TAG => Self::Packet82(Packet82::from_body(&mut cursor)),
            KillCount::TAG => Self::KillCount(KillCount::from_body(&mut cursor)),
            14 => {
                eprintln!("TODO 14");
                Self::Packet49(Packet49 {})
            }
            19 => {
                eprintln!("TODO 19");
                Self::Packet49(Packet49 {})
            }
            20 => {
                eprintln!("TODO 20");
                Self::Packet49(Packet49 {})
            }
            27 => {
                eprintln!("TODO 27");
                Self::Packet49(Packet49 {})
            }
            28 => {
                eprintln!("TODO 28");
                Self::Packet49(Packet49 {})
            }
            29 => {
                eprintln!("TODO 29");
                Self::Packet49(Packet49 {})
            }
            30 => {
                eprintln!("TODO 30");
                Self::Packet49(Packet49 {})
            }
            36 => {
                eprintln!("TODO 36");
                Self::Packet49(Packet49 {})
            }
            39 => {
                eprintln!("TODO 39");
                Self::Packet49(Packet49 {})
            }
            40 => {
                eprintln!("TODO 40");
                Self::Packet49(Packet49 {})
            }
            41 => {
                eprintln!("TODO 41");
                Self::Packet49(Packet49 {})
            }
            45 => {
                eprintln!("TODO 45");
                Self::Packet49(Packet49 {})
            }
            54 => {
                eprintln!("TODO 54");
                Self::Packet49(Packet49 {})
            }
            60 => {
                eprintln!("TODO 60");
                Self::Packet49(Packet49 {})
            }
            74 => {
                eprintln!("TODO 74");
                Self::Packet49(Packet49 {})
            }
            80 => {
                eprintln!("TODO 80");
                Self::Packet49(Packet49 {})
            }
            88 => {
                eprintln!("TODO 88");
                Self::Packet49(Packet49 {})
            }
            98 => {
                eprintln!("TODO 98");
                Self::Packet49(Packet49 {})
            }
            101 => {
                eprintln!("TODO 101");
                Self::Packet49(Packet49 {})
            }
            129 => {
                eprintln!("TODO 129");
                Self::Packet49(Packet49 {})
            }
            136 => {
                eprintln!("TODO 136");
                Self::Packet49(Packet49 {})
            }
            139 => {
                eprintln!("TODO 139");
                Self::Packet49(Packet49 {})
            }
            tag => panic!(format!("unknown tag {}", tag)),
        }
    }
}

#[derive(Debug)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
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

#[derive(Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Serializable for Vec2 {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.x);
        cursor.write(&self.y);
    }
}

impl Deserializable for Vec2 {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        Self {
            x: cursor.read(),
            y: cursor.read(),
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
        Magic {
            magic: "Terraria228".to_string(),
        }
        .serialize(None, &mut cursor);
        let pos = cursor.finish();
        assert_eq!(
            &buffer[..pos],
            &[
                0x0f, 0x00, 0x01, 0x0b, 0x54, 0x65, 0x72, 0x72, 0x61, 0x72, 0x69, 0x61, 0x32, 0x32,
                0x38
            ]
        );
    }
}
