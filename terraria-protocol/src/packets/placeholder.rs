use crate::packets::packet_struct;

packet_struct! {
    /// Placeholder. Does not exist in the official client. Exists solely for
    /// the purpose of being used by custom clients and servers.
    ///
    /// Direction: Variable.
    pub struct Placeholder {
        const TAG = 67;
    }
}
