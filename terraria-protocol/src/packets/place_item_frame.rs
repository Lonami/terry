use crate::serde::packet_struct;

packet_struct! {
    /// Place an item frame.
    ///
    /// Direction: Client -> Server.
    pub struct PlaceItemFrame {
        const TAG = 89;

        pub x: i16,
        pub y: i16,
        pub item_id: i16,
        pub prefix: u8,
        pub stack: i16,
    }
}
