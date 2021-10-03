use crate::serde::packet_struct;
use crate::structures::Vec2;

packet_struct! {
    /// Nebula Level Up.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct NebulaLevelUp {
        const TAG = 102;

        pub player_id: u8,
        pub level_up_type: u16,
        /// In world coordinate pixels
        pub origin: Vec2,
    }
}
