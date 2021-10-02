use crate::packets::packet_struct;

packet_struct! {
    /// Get a chest's name.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct GetChestName {
        const TAG = 69;

        pub id: i16,
        pub x: i16,
        pub y: i16,
        pub name: String,
    }
}
