use crate::packets::packet_struct;
use crate::structures::Vec2;

packet_struct! {
    /// Sync the revenge marker.
    ///
    /// Direction: Server -> Client.
    pub struct SyncRevenge {
        const TAG = 126;

        pub unique_id: i32,
        pub pos: Vec2,
        pub npc_id: i32,
        pub npc_hp_percent: f32,
        pub npc_type: i32,
        pub npc_ai: i32,
        pub coin_value: i32,
        pub base_value: f32,
        pub spawned_from_statue: bool,
    }
}
