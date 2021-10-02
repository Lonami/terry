use crate::packets::packet_struct;

packet_struct! {
    /// Set NPC kill count.
    ///
    /// Direction: Server -> Client.
    pub struct SetNpcKillCount {
        const TAG = 83;

        pub npc_type: i16,
        pub kill_count: i32,
    }
}
