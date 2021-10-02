use crate::packets::packet_struct;

packet_struct! {
    /// Toggle the gem lock.
    ///
    /// Direction: Client -> Server.
    pub struct GemLockToggle {
        const TAG = 105;

        pub x: i16,
        pub y: i16,
        pub on: bool,
    }
}
