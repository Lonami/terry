use crate::serde::packet_struct;

packet_struct! {
    /// Time.
    ///
    /// Direction: Server -> Client.
    pub struct Time {
        const TAG = 18;

        pub day_time: bool,
        pub time: i32,
        pub sun_mod_y: i16,
        pub moon_mod_y: i16,
    }
}
