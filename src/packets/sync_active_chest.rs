use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Sync Active Chest.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct SyncActiveChest {
    pub chest_id: i16,
    pub chest_x: i16,
    pub chest_y: i16,
    pub name_length: u8,
    /// Only if length &gt; 0 &amp;&amp; &lt;= 20
    pub chest_name: String,
}

impl PacketBody for SyncActiveChest {
    const TAG: u8 = 33;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.chest_id);
        cursor.write(&self.chest_x);
        cursor.write(&self.chest_y);
        cursor.write(&self.name_length);
        cursor.write(&self.chest_name);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            chest_id: cursor.read(),
            chest_x: cursor.read(),
            chest_y: cursor.read(),
            name_length: cursor.read(),
            chest_name: cursor.read(),
        }
    }
}
