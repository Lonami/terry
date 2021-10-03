use crate::serde::packet_struct;

packet_struct! {
    /// Land the golf ball in the cup.
    ///
    /// Direction: Client <-> Server.
    pub struct LandGolfBall {
        const TAG = 128;

        pub player_id: u8,
        pub x: i16,
        pub y: i16,
        pub number_of_hits: u16,
        pub proj_id: u16,
    }
}
