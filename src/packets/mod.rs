mod compressed_tile_block;
mod item_moved;
mod kill_count;
mod magic;
mod npc_info;
mod packet100;
mod packet101;
mod packet102;
mod packet103;
mod packet104;
mod packet105;
mod packet106;
mod packet107;
mod packet108;
mod packet109;
mod packet11;
mod packet110;
mod packet111;
mod packet112;
mod packet113;
mod packet114;
mod packet115;
mod packet116;
mod packet117;
mod packet118;
mod packet119;
mod packet120;
mod packet121;
mod packet122;
mod packet123;
mod packet124;
mod packet125;
mod packet126;
mod packet127;
mod packet128;
mod packet129;
mod packet130;
mod packet131;
mod packet132;
mod packet133;
mod packet134;
mod packet135;
mod packet136;
mod packet137;
mod packet138;
mod packet139;
mod packet14;
mod packet15;
mod packet17;
mod packet18;
mod packet19;
mod packet2;
mod packet20;
mod packet22;
mod packet24;
mod packet25;
mod packet26;
mod packet27;
mod packet28;
mod packet29;
mod packet3;
mod packet30;
mod packet31;
mod packet32;
mod packet33;
mod packet34;
mod packet35;
mod packet36;
mod packet37;
mod packet38;
mod packet39;
mod packet40;
mod packet41;
mod packet43;
mod packet44;
mod packet45;
mod packet46;
mod packet47;
mod packet48;
mod packet49;
mod packet51;
mod packet52;
mod packet53;
mod packet54;
mod packet55;
mod packet56;
mod packet57;
mod packet58;
mod packet59;
mod packet6;
mod packet60;
mod packet61;
mod packet62;
mod packet63;
mod packet64;
mod packet65;
mod packet66;
mod packet67;
mod packet69;
mod packet7;
mod packet70;
mod packet71;
mod packet72;
mod packet73;
mod packet74;
mod packet75;
mod packet76;
mod packet77;
mod packet78;
mod packet79;
mod packet8;
mod packet80;
mod packet81;
mod packet82;
mod packet84;
mod packet85;
mod packet86;
mod packet87;
mod packet88;
mod packet89;
mod packet9;
mod packet90;
mod packet91;
mod packet92;
mod packet93;
mod packet94;
mod packet95;
mod packet96;
mod packet97;
mod packet98;
mod packet99;
mod player_buffs;
mod player_info;
mod player_inventory;
mod player_life;
mod player_mana;
mod player_move;
mod player_uuid;
mod to_spawn;

pub use compressed_tile_block::CompressedTileBlock;
pub use item_moved::ItemMoved;
pub use kill_count::KillCount;
pub use magic::Magic;
pub use npc_info::NpcInfo;
pub use packet100::Packet100;
pub use packet101::Packet101;
pub use packet102::Packet102;
pub use packet103::Packet103;
pub use packet104::Packet104;
pub use packet105::Packet105;
pub use packet106::Packet106;
pub use packet107::Packet107;
pub use packet108::Packet108;
pub use packet109::Packet109;
pub use packet11::Packet11;
pub use packet110::Packet110;
pub use packet111::Packet111;
pub use packet112::Packet112;
pub use packet113::Packet113;
pub use packet114::Packet114;
pub use packet115::Packet115;
pub use packet116::Packet116;
pub use packet117::Packet117;
pub use packet118::Packet118;
pub use packet119::Packet119;
pub use packet120::Packet120;
pub use packet121::Packet121;
pub use packet122::Packet122;
pub use packet123::Packet123;
pub use packet124::Packet124;
pub use packet125::Packet125;
pub use packet126::Packet126;
pub use packet127::Packet127;
pub use packet128::Packet128;
pub use packet129::Packet129;
pub use packet130::Packet130;
pub use packet131::Packet131;
pub use packet132::Packet132;
pub use packet133::Packet133;
pub use packet134::Packet134;
pub use packet135::Packet135;
pub use packet136::Packet136;
pub use packet137::Packet137;
pub use packet138::Packet138;
pub use packet139::Packet139;
pub use packet14::Packet14;
pub use packet15::Packet15;
pub use packet17::Packet17;
pub use packet18::Packet18;
pub use packet19::Packet19;
pub use packet2::Packet2;
pub use packet20::Packet20;
pub use packet22::Packet22;
pub use packet24::Packet24;
pub use packet25::Packet25;
pub use packet26::Packet26;
pub use packet27::Packet27;
pub use packet28::Packet28;
pub use packet29::Packet29;
pub use packet3::Packet3;
pub use packet30::Packet30;
pub use packet31::Packet31;
pub use packet32::Packet32;
pub use packet33::Packet33;
pub use packet34::Packet34;
pub use packet35::Packet35;
pub use packet36::Packet36;
pub use packet37::Packet37;
pub use packet38::Packet38;
pub use packet39::Packet39;
pub use packet40::Packet40;
pub use packet41::Packet41;
pub use packet43::Packet43;
pub use packet44::Packet44;
pub use packet45::Packet45;
pub use packet46::Packet46;
pub use packet47::Packet47;
pub use packet48::Packet48;
pub use packet49::Packet49;
pub use packet51::Packet51;
pub use packet52::Packet52;
pub use packet53::Packet53;
pub use packet54::Packet54;
pub use packet55::Packet55;
pub use packet56::Packet56;
pub use packet57::Packet57;
pub use packet58::Packet58;
pub use packet59::Packet59;
pub use packet6::Packet6;
pub use packet60::Packet60;
pub use packet61::Packet61;
pub use packet62::Packet62;
pub use packet63::Packet63;
pub use packet64::Packet64;
pub use packet65::Packet65;
pub use packet66::Packet66;
pub use packet67::Packet67;
pub use packet69::Packet69;
pub use packet7::Packet7;
pub use packet70::Packet70;
pub use packet71::Packet71;
pub use packet72::Packet72;
pub use packet73::Packet73;
pub use packet74::Packet74;
pub use packet75::Packet75;
pub use packet76::Packet76;
pub use packet77::Packet77;
pub use packet78::Packet78;
pub use packet79::Packet79;
pub use packet8::Packet8;
pub use packet80::Packet80;
pub use packet81::Packet81;
pub use packet82::Packet82;
pub use packet84::Packet84;
pub use packet85::Packet85;
pub use packet86::Packet86;
pub use packet87::Packet87;
pub use packet88::Packet88;
pub use packet89::Packet89;
pub use packet9::Packet9;
pub use packet90::Packet90;
pub use packet91::Packet91;
pub use packet92::Packet92;
pub use packet93::Packet93;
pub use packet94::Packet94;
pub use packet95::Packet95;
pub use packet96::Packet96;
pub use packet97::Packet97;
pub use packet98::Packet98;
pub use packet99::Packet99;
pub use player_buffs::PlayerBuffs;
pub use player_info::PlayerInfo;
pub use player_inventory::PlayerInventory;
pub use player_life::PlayerLife;
pub use player_mana::PlayerMana;
pub use player_move::PlayerMove;
pub use player_uuid::PlayerUuid;
pub use to_spawn::ToSpawn;

use crate::serialization::{Deserializable, Serializable, SliceCursor};
use std::convert::TryInto;

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
            17 => {
                eprintln!("TODO 17");
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
            51 => {
                eprintln!("TODO 51");
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
            79 => {
                eprintln!("TODO 79");
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
            103 => {
                eprintln!("TODO 103");
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
