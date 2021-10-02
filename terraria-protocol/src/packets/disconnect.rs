use crate::packets::packet_struct;
use crate::structures::NetString;

packet_struct! {
    /// Disconnect a client (e.g. via kicking).
    ///
    /// Direction: Server -> Client.
    pub struct Disconnect {
        const TAG = 2;

        pub reason: NetString,
    }
}
