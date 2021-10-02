use crate::packets::packet_struct;

packet_struct! {
    /// Update Item Owner.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct UpdateItemOwner {
        const TAG = 22;

        pub item_id: i16,
        pub player_id: u8,
    }
}
