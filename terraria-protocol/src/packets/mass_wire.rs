use crate::packets::packet_struct;

packet_struct! {
    /// Perform a wire operation en-mass.
    ///
    /// Direction: Client -> Server.
    pub struct MassWire {
        const TAG = 109;

        pub start_x: i16,
        pub start_y: i16,
        pub end_x: i16,
        pub end_y: i16,
        /// BitFlags: 1 = Red, 2 = Green, 4 = Blue, 8 = Yellow, 16 = Actuator, 32 = Cutter
        pub toolmode: u8,
    }
}
