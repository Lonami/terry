use crate::packets::packet_struct;

packet_struct! {
    /// Tries to place food in a food platter.
    ///
    /// Direction: Client -> Server.
    pub struct PlaceFood {
        const TAG = 133;

        pub x: i16,
        pub y: i16,
        pub item_id: i16,
        pub prefix: u8,
        pub stack: i16,
    }
}
