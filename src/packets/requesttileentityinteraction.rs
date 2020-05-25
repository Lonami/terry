use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// RequestTileEntityInteraction.
///
/// Direction: Client <-> Server.
#[derive(Debug)]
pub struct RequestTileEntityInteraction {
    pub tileentityid: i32,
    pub player_id: u8,
}

impl PacketBody for RequestTileEntityInteraction {
    const TAG: u8 = 122;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.tileentityid);
        cursor.write(&self.player_id);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            tileentityid: cursor.read(),
            player_id: cursor.read(),
        }
    }
}
