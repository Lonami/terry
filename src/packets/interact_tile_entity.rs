use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Request interaction with a tile entity.
///
/// Direction: Client <-> Server.
#[derive(Debug)]
pub struct InteractTileEntity {
    pub tile_entity_id: i32,
    pub player_id: u8,
}

impl PacketBody for InteractTileEntity {
    const TAG: u8 = 122;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.tile_entity_id);
        cursor.write(&self.player_id);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            tile_entity_id: cursor.read(),
            player_id: cursor.read(),
        }
    }
}
