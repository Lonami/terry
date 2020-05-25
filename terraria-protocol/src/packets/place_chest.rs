use crate::packets::PacketBody;
use crate::SliceCursor;

/// Place a chest.
///
/// Direction: Server <-> Client.
#[derive(Debug)]
pub struct PlaceChest {
    /// BitFlags:0 = Place Chest, 1 = Kill Chest, 2 = Place Dresser, 3 = Kill Dresser. 4 = Place Containers2, 5 = Kill Containers2
    pub action: u8,
    pub tile_x: i16,
    pub tile_y: i16,
    /// FrameX(Chest type)
    pub style: i16,
    /// ID if client is receiving packet, else 0
    pub chest_id_to_destroy: i16,
}

impl PacketBody for PlaceChest {
    const TAG: u8 = 34;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.action);
        cursor.write(&self.tile_x);
        cursor.write(&self.tile_y);
        cursor.write(&self.style);
        cursor.write(&self.chest_id_to_destroy);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            action: cursor.read(),
            tile_x: cursor.read(),
            tile_y: cursor.read(),
            style: cursor.read(),
            chest_id_to_destroy: cursor.read(),
        }
    }
}
