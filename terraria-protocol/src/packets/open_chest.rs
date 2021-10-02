use crate::packets::packet_struct;

packet_struct! {
    /// Used to "open" a world chest (that is, an item container placed in the
    /// world). When this packet is received the server will send the chest's
    /// contents and sync the active chest ID to the player using packet 33
    /// `SyncActiveChest`.
    ///
    /// Direction: Client -> Server.
    pub struct OpenChest {
        const TAG = 31;

        pub tile_x: i16,
        pub tile_y: i16,
    }
}
