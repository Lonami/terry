use crate::serde::packet_struct;

packet_struct! {
    /// Toggle a birthday party.
    ///
    /// Direction: Client -> Server.
    pub struct ToggleBirthdayParty {
        const TAG = 111;
    }
}
