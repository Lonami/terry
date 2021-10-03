use crate::packets::packet_struct;
use crate::structures::serializable_bitflags;

serializable_bitflags! {
    pub struct SoundEffect: u8 {
        const TREE_GROWTH = 0x01;
        const FAIRY = 0x02;
    }
}

packet_struct! {
    /// Growth sound effects.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct GrowFx {
        const TAG = 112;

        pub effect: SoundEffect,
        pub x: i32,
        pub y: i32,
        pub height: u8,
        pub tree_gore: i16,
    }
}
