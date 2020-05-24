use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Player UUID-4.
pub struct PlayerUuid {
    pub uuid4: String,
}

impl PacketBody for PlayerUuid {
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
