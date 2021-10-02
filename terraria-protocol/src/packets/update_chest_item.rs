use crate::packets::packet_struct;

packet_struct! {
    /// Update Chest Item.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct UpdateChestItem {
        const TAG = 32;

        pub chest_id: i16,
        pub item_slot: u8,
        pub stack: i16,
        pub prefix: u8,
        pub item_net_id: i16,
    }
}
