use crate::packets::packet_struct;

packet_struct! {
    /// Poof of smoke.
    ///
    /// Direction: Server -> Client.
    pub struct PoofOfSmoke {
        const TAG = 106;

        pub x: i16,
        pub y: i16,
    }
}
