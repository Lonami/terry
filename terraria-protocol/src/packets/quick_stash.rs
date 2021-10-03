use crate::serde::packet_struct;

packet_struct! {
    /// Force an item into the nearest chest (quick stashing).
    ///
    /// Direction: Client -> Server.
    pub struct QuickStash {
        const TAG = 85;

        pub inventory_slot: u8,
    }
}
