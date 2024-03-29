use crate::serde::packet_struct;

packet_struct! {
    /// Request password.
    ///
    /// Direction: Server -> Client.
    pub struct RequestPassword {
        const TAG = 37;
    }
}
