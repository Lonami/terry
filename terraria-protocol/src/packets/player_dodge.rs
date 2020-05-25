use crate::packets::PacketBody;
use crate::SliceCursor;

/// Player dodging.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct PlayerDodge {
    pub player_id: u8,
    /// 1 = Ninja Dodge 2 = Shadow Dodge
    pub flag: u8,
}

impl PacketBody for PlayerDodge {
    const TAG: u8 = 62;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.flag);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            flag: cursor.read(),
        }
    }
}
