use crate::packets::Packet;
use crate::serialization::SliceCursor;

/// Magic sent at the very beginning of the communication.
pub struct Magic {
    pub magic: String,
}

impl Packet for Magic {
    const TAG: u8 = 1;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.magic);
    }

    fn from_body(&self, cursor: &mut SliceCursor) -> Self {
        Self {
            magic: cursor.read(),
        }
    }
}
