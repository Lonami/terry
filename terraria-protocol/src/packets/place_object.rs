use crate::packets::packet_struct;

packet_struct! {
    /// Place an object.
    ///
    /// Direction: Server <-> Client.
    pub struct PlaceObject {
        const TAG = 79;

        pub x: i16,
        pub y: i16,
        pub ty: i16,
        pub style: i16,
        pub alternate: u8,
        pub random: i8,
        pub direction: bool,
    }
}
