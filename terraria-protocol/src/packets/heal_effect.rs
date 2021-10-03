use crate::serde::packet_struct;

packet_struct! {
    /// Apply a healing effect.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct HealEffect {
        const TAG = 35;

        pub player_id: u8,
        pub heal_amount: i16,
    }
}
