use crate::packets::packet_struct;

packet_struct! {
    /// Display doll item sync.
    ///
    /// Direction: Client <-> Server.
    pub struct DollSync {
        const TAG = 121;

        pub player_id: u8,
        pub tile_entity_id: i32,
        pub item_index: u8,
        pub item_id: u16,
        pub stack: u16,
        pub prefix: u8,
    }
}
