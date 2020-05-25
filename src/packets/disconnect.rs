use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Disconnect a client (e.g. via kicking).
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct Disconnect {
    pub reason: NetString,
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
