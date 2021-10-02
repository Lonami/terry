use crate::packets::packet_struct;

packet_struct! {
    /// Request to remove a NPC's buffs.
    ///
    /// Direction: Client -> Server.
    pub struct RequestNpcDebuff {
        const TAG = 137;

        pub npc_id: i16,
        pub buff_id: u16,
    }
}
