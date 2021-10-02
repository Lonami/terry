use crate::packets::packet_struct;

packet_struct! {
    /// Add a buff (or debuff) to some NPC for a certain duration.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct AddNpcBuff {
        const TAG = 53;

        pub npc_id: i16,
        pub buff: u16,
        pub time: i16,
    }
}
