use crate::packets::packet_struct;

packet_struct! {
    /// Unlock.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct Unlock {
        const TAG = 52;

        /// Values: 1 = Chest Unlock, 2 = Door Unlock
        pub ty: u8,
        pub x: i16,
        pub y: i16,
    }
}
