use crate::serde::packet_struct;

packet_struct! {
    /// Player health points and maximum HP.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct PlayerHp {
        const TAG = 16;

        pub player_id: u8,
        pub hp: i16,
        pub max_hp: i16,
    }
}
