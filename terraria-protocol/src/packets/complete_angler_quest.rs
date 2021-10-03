use crate::serde::packet_struct;

packet_struct! {
    /// Complete the Angler quest today.
    ///
    /// Direction: Client -> Server.
    pub struct CompleteAnglerQuest {
        const TAG = 75;
    }
}
