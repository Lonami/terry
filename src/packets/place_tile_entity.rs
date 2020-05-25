use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Place Tile Entity.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct PlaceTileEntity {
    pub x: i16,
    pub y: i16,
    /// 2 = Logic Sensor 1 = Item Frame 0 = Training Dummy
    pub tileentitytype: u8,
}

impl PacketBody for PlaceTileEntity {
    const TAG: u8 = 87;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.x);
        cursor.write(&self.y);
        cursor.write(&self.tileentitytype);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            x: cursor.read(),
            y: cursor.read(),
            tileentitytype: cursor.read(),
        }
    }
}
