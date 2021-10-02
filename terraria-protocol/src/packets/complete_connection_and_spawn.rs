use crate::packets::packet_struct;

packet_struct! {
    /// Complete the connection process and spawn.
    ///
    /// Direction: Server -> Client.
    pub struct CompleteConnectionAndSpawn {
        const TAG = 49;
    }
}
