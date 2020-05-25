use crate::packets::PacketBody;
use crate::SliceCursor;

/// Update Tile Entity.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct UpdateTileEntity {
    pub tileentityid: i32,
    /// If UpdateTileFlag is false, TileEntity is removed
    pub updatetileflag: bool,
    /// Only sent if UpdateTileFlag is false
    pub tileentity_type: u8,
    /// Only sent if UpdateTileFlag is false
    pub x: i16,
    /// Only sent if UpdateTileFlag is false
    pub y: i16,
}

impl PacketBody for UpdateTileEntity {
    const TAG: u8 = 86;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.tileentityid);
        cursor.write(&self.updatetileflag);
        cursor.write(&self.tileentity_type);
        cursor.write(&self.x);
        cursor.write(&self.y);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            tileentityid: cursor.read(),
            updatetileflag: cursor.read(),
            tileentity_type: cursor.read(),
            x: cursor.read(),
            y: cursor.read(),
        }
    }
}
