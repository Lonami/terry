use crate::serde::{PacketBody, Result, SliceCursor};
use crate::structures::{Chest, Sign, Tile, TileEntity};
use inflate;

/// Send section.
///
/// Direction: Server -> Client.
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SendSection {
    pub x_start: i32,
    pub y_start: i32,
    pub width: i16,
    pub height: i16,
    pub tiles: Vec<Tile>,
    pub chests: Vec<Chest>,
    pub signs: Vec<Sign>,
    pub tile_entities: Vec<TileEntity>,
}

fn read_decompressed_section(cursor: &mut SliceCursor) -> Result<SendSection> {
    let x_start = cursor.read()?;
    let y_start = cursor.read()?;
    let width = cursor.read()?;
    let height = cursor.read()?;

    let mut tiles: Vec<Tile> = Vec::with_capacity((width * height) as usize);
    let mut rle = 0; // kind of a run-length encoding

    for _ in 0..width * height {
        if rle != 0 {
            rle -= 1;
            tiles.push(tiles[tiles.len() - 1].clone());
        } else {
            // tiles are encoded differently when being read through here
            let tup = Tile::deserialize_packed(cursor)?;
            tiles.push(tup.0);
            rle = tup.1;
        }
    }

    let n = cursor.read::<u16>()? as usize;
    let chests = (0..n).map(|_| cursor.read()).collect::<Result<_>>()?;

    let n = cursor.read::<u16>()? as usize;
    let signs = (0..n).map(|_| cursor.read()).collect::<Result<_>>()?;

    let n = cursor.read::<u16>()? as usize;
    let tile_entities = (0..n).map(|_| cursor.read()).collect::<Result<_>>()?;

    Ok(SendSection {
        x_start,
        y_start,
        width,
        height,
        tiles,
        chests,
        signs,
        tile_entities,
    })
}

impl PacketBody for SendSection {
    const TAG: u8 = 10;

    fn write_body(&self, _cursor: &mut SliceCursor) -> Result<()> {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Result<Self> {
        if cursor.read::<bool>()? {
            let mut decompressed =
                inflate::inflate_bytes(cursor.read_to_end()).expect("failed to decompress tiles");

            // I can't make the borrow checker happy because if I tried to
            // update cursor to this new borrowed data (even if defined out)
            // would have a different lifetime, and it won't work with that
            // mismatch. So instead we use a separate function.
            read_decompressed_section(&mut SliceCursor::new(decompressed.as_mut_slice()))
        } else {
            read_decompressed_section(cursor)
        }
    }
}
