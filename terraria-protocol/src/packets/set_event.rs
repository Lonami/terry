use crate::serde::packet_struct;

packet_struct! {
    /// Notifies the player of an event.
    ///
    /// Direction: Server -> Client.
    pub struct SetEvent {
        const TAG = 98;

        pub event_id: i16,
    }
}
