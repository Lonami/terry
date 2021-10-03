use crate::serde::packet_struct;

packet_struct! {
    /// Information about Angler quests.
    ///
    /// Direction: Server -> Client.
    pub struct AnglerQuest {
        const TAG = 74;

        pub quest: u8,
        pub completed: bool,
    }
}
