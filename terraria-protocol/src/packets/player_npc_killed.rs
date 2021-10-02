use crate::packets::packet_struct;

packet_struct! {
    /// Notifies when a player's NPC is killed.
    ///
    /// Direction: Server -> Client.
    pub struct PlayerNpcKilled {
        const TAG = 97;

        pub npc_id: i16,
    }
}
