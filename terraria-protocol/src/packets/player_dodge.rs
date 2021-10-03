use crate::serde::{packet_struct, serializable_enum};

serializable_enum! {
    pub enum DodgeType: u8 {
        Ninja = 1,
        Shadow = 2,
    }
}

packet_struct! {
    /// Player dodging.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct PlayerDodge {
        const TAG = 62;

        pub player_id: u8,
        pub ty: DodgeType,
    }
}
