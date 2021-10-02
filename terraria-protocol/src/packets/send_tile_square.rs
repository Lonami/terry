use crate::packets::PacketBody;
use crate::structures::Tile;
use crate::SliceCursor;

/// Send a tile square.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct SendTileSquare {
    pub tile_y: i16,
    pub tile_x: i16,
    pub height: u8,
    pub width: u8,
    pub change_type: u8,
    pub tiles: Vec<Tile>,
}

impl PacketBody for SendTileSquare {
    const TAG: u8 = 20;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        let tile_y = cursor.read::<i16>();
        let tile_x = cursor.read::<i16>();
        let height = cursor.read::<u8>();
        let width = cursor.read::<u8>();
        let change_type = cursor.read::<u8>(); // none, lavawater, honeywater, honeylava

        let mut tiles: Vec<Tile> = Vec::with_capacity((width * height) as usize);

        (0..width * height).for_each(|_| {
            tiles.push(cursor.read());
        });

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
