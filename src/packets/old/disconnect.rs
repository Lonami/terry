use crate::packets::{NetString, PacketBody};
use crate::serialization::SliceCursor;

/// Disconnect, sent prior to closing the connection.
///
/// Direction: Server to Client.
#[derive(Debug)]
pub struct Disconnect {
    reason: NetString,
}

impl PacketBody for Disconnect {
    const TAG: u8 = 2;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.reason);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            reason: cursor.read(),
        }
    }
}
