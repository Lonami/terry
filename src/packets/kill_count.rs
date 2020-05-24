use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Kill count to a certain NPC.
#[derive(Debug)]
pub struct KillCount {
    pub id: u16,
    pub count: u16,
}

impl PacketBody for KillCount {
    const TAG: u8 = 83;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.id);
        cursor.write(&self.count);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            id: cursor.read(),
            count: cursor.read(),
        }
    }
}
