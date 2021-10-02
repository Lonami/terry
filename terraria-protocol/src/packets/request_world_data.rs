use crate::packets::packet_struct;

packet_struct! {
    /// Request world data.
    ///
    /// Direction: Client -> Server.
    pub struct RequestWorldData {
        const TAG = 6;
    }
}
