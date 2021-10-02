use crate::packets::packet_struct;

packet_struct! {
    /// Number of Angler quests completed.
    ///
    /// Direction: Client -> Server.
    pub struct AnglerQuests {
        const TAG = 76;

        pub player_id: u8,
        pub angler_quests_completed: i32,
        pub golfer_score: i32,
    }
}
