use crate::serde::packet_struct;

packet_struct! {
    /// Request a sign.
    ///
    /// Direction: Client -> Server.
    pub struct RequestSign {
        const TAG = 46;

        pub x: i16,
        pub y: i16,
    }
}
