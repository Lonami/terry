use crate::packets::packet_struct;

packet_struct! {
    /// Try placing in a weapon rack.
    ///
    /// Direction: Client -> Server.
    pub struct PlaceWeaponRack {
        const TAG = 123;

        pub x: i16,
        pub y: i16,
        pub net_id: i16,
        pub prefix: u8,
        pub stack: i16,
    }
}
