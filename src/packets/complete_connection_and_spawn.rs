use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Complete Connection and Spawn.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct CompleteConnectionandSpawn {
    pub -: (),
}

impl PacketBody for CompleteConnectionandSpawn {
    const TAG: u8 = 49;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.-);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            -: cursor.read(),
        }
    }
}
