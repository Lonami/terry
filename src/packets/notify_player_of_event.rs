use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Notify Player Of Event.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct NotifyPlayerOfEvent {
    pub event_id: i16,
}

impl PacketBody for NotifyPlayerOfEvent {
    const TAG: u8 = 98;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.event_id);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            event_id: cursor.read(),
        }
    }
}
