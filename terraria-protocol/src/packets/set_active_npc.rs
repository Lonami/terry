use crate::packets::packet_struct;

packet_struct! {
    /// Set the active NPC.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct SetActiveNpc {
        const TAG = 40;

        pub player_id: u8,
        pub npc_talk_target: i16,
    }
}
