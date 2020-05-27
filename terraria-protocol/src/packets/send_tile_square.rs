use crate::packets::PacketBody;
use crate::structures::Tile;
use crate::SliceCursor;
use std::convert::TryInto;

/// Send a tile square.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct SendTileSquare {
    pub tile_change_type: Option<u8>,
    pub tile_x: i16,
    pub tile_y: i16,
    pub tiles: Vec<Tile>,
}

impl PacketBody for SendTileSquare {
    const TAG: u8 = 20;

    fn write_body(&self, cursor: &mut SliceCursor) {
        let mut size: u16 = self.tiles.len().try_into().expect("too many tiles");
        if self.tile_change_type.is_some() {
            size |= 0x8000;
        }
        cursor.write::<u16>(&size);
        if let Some(ty) = self.tile_change_type.as_ref() {
            cursor.write(ty);
        }
        cursor.write(&self.tile_x);
        cursor.write(&self.tile_y);
        self.tiles.iter().for_each(|t| cursor.write(t));
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        let mut size: u16 = cursor.read();

        let tile_change_type = if size & 0x8000 != 0 {
            size &= 0x7fff;
            Some(cursor.read())
        } else {
            None
        };

        let tile_x = cursor.read();
        let tile_y = cursor.read();

        let mut tiles = Vec::with_capacity(size as usize);
        (0..size).for_each(|_| tiles.push(cursor.read()));

        Self {
            tile_change_type,
            tile_x,
            tile_y,
            tiles,
        }
    }
}
