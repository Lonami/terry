use crate::packets::packet_struct;

packet_struct! {
    /// Sync tile picking.
    ///
    /// Direction: Client <-> Server.
    pub struct SyncTilePicking {
        const TAG = 125;

        pub player_id: u8,
        pub x: i16,
        pub y: i16,
        pub pick_damage: u8,
    }
}
