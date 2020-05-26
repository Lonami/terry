use crate::packets::PacketBody;
use crate::SliceCursor;

/// Client UUID.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct ClientUuid {
    pub uuid4: String,
}

impl PacketBody for ClientUuid {
    const TAG: u8 = 68;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.uuid4);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            uuid4: cursor.read(),
        }
    }
}
