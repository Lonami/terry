use crate::packets::packet_struct;

packet_struct! {
    /// Player mana and maximum mana.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct PlayerMana {
        const TAG = 42;

        pub player_id: u8,
        pub mana: i16,
        pub max_mana: i16,
    }
}
