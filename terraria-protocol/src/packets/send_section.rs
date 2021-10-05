use crate::serde::{Error, PacketBody, Result, SliceCursor};
use crate::structures::{Chest, Sign, Tile, TileEntity};
use inflate;
use std::fmt;

const ABSURD_SIZE: usize = 1000 * 1000;

/// Send section.
///
/// Direction: Server -> Client.
#[derive(PartialEq, Eq, Default, Clone)]
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

    if width < 0 || height < 0 {
        return Err(Error::MalformedPayload {
            details: format!("Invalid section dimensions: {}x{}", width, height),
        });
    }

    let dimensions = (width as usize) * (height as usize);
    if dimensions > ABSURD_SIZE {
        return Err(Error::MalformedPayload {
            details: format!("Section dimensions are far too large: {}x{}", width, height),
        });
    }

    let mut tiles: Vec<Tile> = Vec::with_capacity(dimensions);
    let mut rle = 0; // kind of a run-length encoding

    for _ in 0..dimensions {
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
            let mut decompressed = inflate::inflate_bytes(cursor.read_to_end())
                .map_err(|details| Error::MalformedPayload { details })?;

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

impl fmt::Debug for SendSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SendSection")
            .field("x_start", &self.x_start)
            .field("y_start", &self.y_start)
            .field("width", &self.width)
            .field("height", &self.height)
            // no tiles, cmd would be printing them for a long while
            .field("chests", &self.chests)
            .field("signs", &self.signs)
            .field("tile_entities", &self.tile_entities)
            .finish_non_exhaustive()
    }
}
