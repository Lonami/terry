use crate::serde::packet_struct;

packet_struct! {
    /// Client UUID.
    ///
    /// Direction: Client -> Server.
    pub struct ClientUuid {
        const TAG = 68;

        pub uuid4: String,
    }
}
