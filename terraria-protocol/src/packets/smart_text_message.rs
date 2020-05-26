use crate::packets::PacketBody;
use crate::structures::{NetString, RGB};
use crate::SliceCursor;
use std::convert::TryInto;

/// Smart Text Message (FKA. Chat Message v2).
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct SmartTextMessage {
    /// Note that the client cannot change colors.
    pub message_color: RGB,
    pub message: NetString,
}

impl PacketBody for SmartTextMessage {
    const TAG: u8 = 107;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.message_color);
        cursor.write(&self.message);
        let message_len: u16 = self.message.len().try_into().expect("message too long");
        cursor.write(&message_len); // TODO not sure why this is here? NetString knows len already
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            message_color: cursor.read(),
            message: cursor.read(),
        }
    }
}
