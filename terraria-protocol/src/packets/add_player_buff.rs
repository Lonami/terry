use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Add a buff (or debuff) to some player for a certain duration.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct AddPlayerBuff {
    pub player_id: u8,
    pub buff: u16,
    pub time: i32,
}

impl PacketBody for AddPlayerBuff {
    const TAG: u8 = 55;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.buff);
        cursor.write(&self.time);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            buff: cursor.read(),
            time: cursor.read(),
        }
    }
}
