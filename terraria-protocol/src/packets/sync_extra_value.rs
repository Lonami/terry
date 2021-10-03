use crate::serde::packet_struct;
use crate::structures::Vec2;

packet_struct! {
    /// Sync an extra value.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct SyncExtraValue {
        const TAG = 92;

        pub npc_index: u16,
        pub extra_value: i32,
        pub pos: Vec2,
    }
}
