use crate::packets::packet_struct;

packet_struct! {
    /// Remove item owner.
    ///
    /// Direction: Server -> Client.
    pub struct RemoveItemOwner {
        const TAG = 39;

        pub item_index: i16,
    }
}
