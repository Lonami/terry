use crate::serde::packet_struct;

packet_struct! {
    /// Social handshake.
    ///
    /// Direction: Not used.
    pub struct SocialHandshake {
        const TAG = 93;
    }
}
