use crate::serde::{serializable_enum, PacketBody, SliceCursor};
use crate::structures::Tile;

serializable_enum! {
    pub enum ChangeType: u8 {
        None = 0,
        LavaWater = 1,
        HoneyWater = 2,
        HoneyLava = 3,
    }
}

/// Send a tile square.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SendTileSquare {
    pub tile_y: i16,
    pub tile_x: i16,
    pub height: u8,
    pub width: u8,
    pub change_type: ChangeType,
    pub tiles: Vec<Tile>,
}

impl PacketBody for SendTileSquare {
    const TAG: u8 = 20;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.tile_y);
        cursor.write(&self.tile_x);
        cursor.write(&self.height);
        cursor.write(&self.width);
        cursor.write(&self.change_type);
        self.tiles.iter().for_each(|tile| cursor.write(tile));
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        let tile_y = cursor.read();
        let tile_x = cursor.read();
        let height = cursor.read();
        let width = cursor.read();
        let change_type = cursor.read();
        let tiles = (0..width * height).map(|_| cursor.read()).collect();

        Self {
            tile_y,
            tile_x,
            height,
            width,
            change_type,
            tiles,
        }
    }
}
