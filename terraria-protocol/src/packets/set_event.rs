use crate::packets::PacketBody;
use crate::SliceCursor;

/// Notifies the player of an event.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct SetEvent {
    pub event_id: i16,
}

impl PacketBody for SetEvent {
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
