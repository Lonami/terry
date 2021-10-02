use crate::packets::packet_struct;

packet_struct! {
    /// Send password.
    ///
    /// Direction: Client -> Server.
    pub struct SendPassword {
        const TAG = 38;

        pub password: String,
    }
}
