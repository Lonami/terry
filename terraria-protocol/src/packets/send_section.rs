use crate::packets::PacketBody;
use crate::structures::{Chest, Sign, Tile, Liquid, TileEntity};
use crate::SliceCursor;
use inflate;

/// Send section.
///
/// Direction: Server -> Client.
#[derive(Debug)]
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

fn read_decompressed_section(cursor: &mut SliceCursor) -> SendSection {
    let x_start = cursor.read();
    let y_start = cursor.read();
    let width = cursor.read();
    let height = cursor.read();

    let mut tiles: Vec<Tile> = Vec::with_capacity((width * height) as usize);

    let mut rle = 0i16; // kind of a run-length encoding
    for _ in y_start..y_start + height as i32 {
        for _ in x_start..x_start + width as i32 {
            if rle != 0 {
                rle -= 1;
                tiles.push(tiles[tiles.len() - 1].clone());
            } else {
                let tile = cursor.read::<Tile>();
                rle = tile.rle;
                tiles.push(tile);
            }
        }
    }

    eprintln!(">>");
    tiles.iter().for_each(|t| if t.ty.is_some() {
        dbg!(t);
    });
    eprintln!("<<");

    let mut chests = Vec::new();
    chests = Vec::with_capacity(cursor.read::<u16>() as usize);
    (0..chests.capacity()).for_each(|_| chests.push(cursor.read()));

    let mut signs = Vec::new();
    signs = Vec::with_capacity(cursor.read::<u16>() as usize);
    (0..signs.capacity()).for_each(|_| signs.push(cursor.read()));

    let mut tile_entities = Vec::new();
    tile_entities = Vec::with_capacity(cursor.read::<u16>() as usize);
    (0..tile_entities.capacity()).for_each(|_| tile_entities.push(cursor.read()));

    SendSection {
        x_start,
        y_start,
        width,
        height,
        tiles,
        chests,
        signs,
        tile_entities,
    }
}

impl PacketBody for SendSection {
    const TAG: u8 = 10;

    fn write_body(&self, _cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // compressed?
        if cursor.read::<bool>() {
            let mut decompressed =
                inflate::inflate_bytes(cursor.read_to_end()).expect("failed to decompress tiles");

            // I can't make the borrow checker happy because if I tried to
            // update cursor to this new borrowed data (even if defined out)
            // would have a different lifetime, and it won't work with that
            // mismatch. So instead we use a separate function.
            eprintln!("-> decompressed: {:?}", decompressed);
            read_decompressed_section(&mut SliceCursor::new(decompressed.as_mut_slice()))
        } else {
            read_decompressed_section(cursor)
        }
    }
}
