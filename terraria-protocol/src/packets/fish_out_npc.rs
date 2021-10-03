use crate::packets::packet_struct;

packet_struct! {
    /// Fish out a NPC.
    ///
    /// Direction: Client -> Server.
    pub struct FishOutNpc {
        const TAG = 130;

        pub x: i16,
        pub y: i16,
        pub npc_id: i16,
    }
}
