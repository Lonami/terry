use crate::packets::packet_struct;

packet_struct! {
    /// Remove the revenge marker.
    ///
    /// Direction: Server -> Client.
    pub struct RemoveRevenge {
        const TAG = 127;

        pub unique_id: i32,
    }
}
