use crate::serde::packet_struct;

packet_struct! {
    /// Consume wires en-mass.
    ///
    /// Direction: Server -> Client.
    pub struct MassConsumeWire {
        const TAG = 110;

        pub item_type: i16,
        pub quantity: i16,
        pub player_id: u8,
    }
}
