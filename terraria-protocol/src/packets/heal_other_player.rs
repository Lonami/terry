use crate::serde::packet_struct;

packet_struct! {
    /// Apply a healing effect to other player.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct HealOtherPlayer {
        const TAG = 66;

        pub player_id: u8,
        pub heal_amount: i16,
    }
}
