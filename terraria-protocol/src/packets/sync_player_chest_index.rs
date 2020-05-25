use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Sync a player chest iIndex.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct SyncPlayerChestIndex {
    pub player: u8,
    pub chest: i16,
}

impl PacketBody for SyncPlayerChestIndex {
    const TAG: u8 = 80;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player);
        cursor.write(&self.chest);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player: cursor.read(),
            chest: cursor.read(),
        }
    }
}
