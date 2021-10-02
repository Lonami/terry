use crate::packets::packet_struct;
use crate::structures::Vec2;

packet_struct! {
    /// Update Minion Target.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct UpdateMinionTarget {
        const TAG = 99;

        pub player_id: u8,
        pub target: Vec2,
    }
}
