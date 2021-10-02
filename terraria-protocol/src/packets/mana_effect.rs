use crate::packets::packet_struct;

packet_struct! {
    /// Apply a mana healing effect.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct ManaEffect {
        const TAG = 43;

        pub player_id: u8,
        pub mana_amount: i16,
    }
}
