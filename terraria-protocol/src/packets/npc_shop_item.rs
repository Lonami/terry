use crate::serde::packet_struct;

packet_struct! {
    /// The item from a NPC shop.
    ///
    /// Direction: Server -> Client.
    pub struct NpcShopItem {
        const TAG = 104;

        pub slot: u8,
        pub item_type: i16,
        pub stack: i16,
        pub prefix: u8,
        pub value: i32,
        pub buy_once: bool,
    }
}
