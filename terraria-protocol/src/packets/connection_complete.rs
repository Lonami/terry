use crate::serde::packet_struct;

packet_struct! {
    /// Finished connecting to the server.
    ///
    /// Direction: Server -> Client.
    pub struct ConnectionComplete {
        const TAG = 129;
    }
}
