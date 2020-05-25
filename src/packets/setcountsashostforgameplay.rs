use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// SetCountsAsHostForGameplay.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct SetCountsAsHostForGameplay {
    pub player_id: u8,
    pub countsashost: bool,
}

impl PacketBody for SetCountsAsHostForGameplay {
    const TAG: u8 = 139;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.countsashost);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            countsashost: cursor.read(),
        }
    }
}
