use crate::serde::packet_struct;

packet_struct! {
    /// Strike a NPC with the held item.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct StrikeNpc {
        const TAG = 24;

        pub npc_id: i16,
        pub player_id: u8,
    }
}
