use crate::serde::packet_struct;

packet_struct! {
    /// Set liquid.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct SetLiquid {
        const TAG = 48;

        pub x: i16,
        pub y: i16,
        pub liquid: u8,
        pub liquid_type: u8,
    }
}
