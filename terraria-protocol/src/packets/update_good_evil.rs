use crate::packets::packet_struct;

packet_struct! {
    /// Update Good Evil.
    ///
    /// Direction: Server -> Client.
    pub struct UpdateGoodEvil {
        const TAG = 57;

        pub good: u8,
        pub evil: u8,
        pub crimson: u8,
    }
}
