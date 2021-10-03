use crate::packets::packet_struct;
use crate::structures::serializable_enum;

serializable_enum! {
    pub enum UnlockType: u8 {
        Chest = 1,
        Door = 2,
    }
}

packet_struct! {
    /// Unlock.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct Unlock {
        const TAG = 52;

        pub ty: UnlockType,
        pub x: i16,
        pub y: i16,
    }
}
