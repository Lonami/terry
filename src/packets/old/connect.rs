use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Connect request, sent at the very beginning of the communication.
///
/// Direction: Client to Server.
#[derive(Debug)]
pub struct Connect {
    pub version: String,
}

impl PacketBody for Connect {
    const TAG: u8 = 1;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.version);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            version: cursor.read(),
        }
    }
}
