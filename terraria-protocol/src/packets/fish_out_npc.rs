use crate::packets::packet_struct;

packet_struct! {
    /// Fish out a NPC.
    ///
    /// Direction: Client -> Server.
    pub struct FishOutNpc {
        const TAG = 130;

        pub x: u16,
        pub y: u16,
        pub npc_id: i16,
    }
}
