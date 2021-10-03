use crate::packets::packet_struct;

packet_struct! {
    /// Catch a NPC, presumably used for pets.
    ///
    /// Direction: Client -> Server.
    pub struct CatchNpc {
        const TAG = 70;

        pub npc_id: i16,
        pub player_id: u8,
    }
}
