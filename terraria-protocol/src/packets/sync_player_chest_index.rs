use crate::packets::packet_struct;

packet_struct! {
    /// Sync a player chest iIndex.
    ///
    /// Direction: Server -> Client.
    pub struct SyncPlayerChestIndex {
        const TAG = 80;

        pub player: u8,
        pub chest: i16,
    }
}
