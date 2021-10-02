use crate::packets::packet_struct;

packet_struct! {
    /// Updates a NPC's home.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct SetNpcHome {
        const TAG = 60;

        pub npc_id: i16,
        pub home_tile_x: i16,
        pub home_tile_y: i16,
        pub homeless: u8,
    }
}
