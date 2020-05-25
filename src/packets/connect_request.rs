use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Connect request.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct ConnectRequest {
    /// "Terraria" + Main.curRelease
    pub version: String,
}

impl PacketBody for ConnectRequest {
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
