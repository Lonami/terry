use crate::packets::{PacketBody, Tile, Chest, Sign, TileEntity};
use crate::serialization::SliceCursor;

/// A region of tiles in the world.
///
/// Direction: Server to Client.
#[derive(Debug)]
pub struct TileSection {
    pub compressed: bool,
    pub x: u32,
    pub y: u32,
    pub width: u16,
    pub height: u16,
    pub tiles: Vec<Tile>,
    pub chests: Vec<Chest>, // pre: chest_count: u16,
    pub signs: Vec<Sign>, // pre: sign_count: u16,
    pub entities: Vec<TileEntity>, // pre: tile_entity_count: u16,
}

impl PacketBody for TileSection {
    const TAG: u8 = 10;

    fn write_body(&self, _cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(_cursor: &mut SliceCursor) -> Self {
        todo!()
    }
}
