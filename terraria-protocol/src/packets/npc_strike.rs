use crate::packets::packet_struct;

packet_struct! {
    /// NPC attacks.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct NpcStrike {
        const TAG = 28;

        pub npc_id: i16,
        /// -1 = Kill
        pub damage: i16,
        pub knockback: f32,
        pub hit_direction: u8,
        pub crit: bool,
    }
}
