use crate::serde::packet_struct;

packet_struct! {
    /// Add a buff (or debuff) to some player for a certain duration.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct AddPlayerBuff {
        const TAG = 55;

        pub player_id: u8,
        pub buff: u16,
        pub time: i32,
    }
}
