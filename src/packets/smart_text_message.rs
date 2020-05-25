use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Smart Text Message (FKA. Chat Message v2).
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct SmartTextMessage {
    /// Client cannot change colors
    pub message_color: Color,
}

impl PacketBody for SmartTextMessage {
    const TAG: u8 = 107;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.message_color);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            message_color: cursor.read(),
        }
    }
}
