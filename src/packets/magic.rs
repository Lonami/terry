use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Magic sent at the very beginning of the communication.
#[derive(Debug)]
pub struct Magic {
    pub magic: String,
}

impl PacketBody for Magic {
    const TAG: u8 = 1;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.magic);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            magic: cursor.read(),
        }
    }
}
