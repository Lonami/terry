use crate::packets::PacketBody;
use crate::structures::{Chest, Sign, Tile, TileEntity};
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
    let mut rle = 0; // kind of a run-length encoding

    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(format!("map/{}-{}_{}-{}.ppm", x_start, y_start, width, height)).unwrap();
    file.write_all(format!("P6 {} {} 255\n", width, height).as_bytes()).unwrap();

    (0..width * height).for_each(|_| {
        if rle != 0 {
            rle -= 1;
            tiles.push(tiles[tiles.len() - 1].clone());
        } else {
            let tile = cursor.read::<Tile>();
            rle = tile.rle;
            tiles.push(tile);
        }

        if let Some(ty) = tiles[tiles.len() - 1].ty {
            let rgb = 26200 + (ty as u32) * 26200;
            file.write_all(&rgb.to_le_bytes()[..3]).unwrap();
        } else {
            file.write_all(&[0, 0, 0]).unwrap();
        };
    });

    let n = cursor.read::<u16>() as usize;
    let mut chests = Vec::with_capacity(n);
    (0..n).for_each(|_| chests.push(cursor.read()));

    let n = cursor.read::<u16>() as usize;
    let mut signs = Vec::with_capacity(n);
    (0..n).for_each(|_| signs.push(cursor.read()));

    let n = cursor.read::<u16>() as usize;
    let mut tile_entities = Vec::with_capacity(n);
    (0..n).for_each(|_| tile_entities.push(cursor.read()));

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
        if cursor.read::<bool>() {
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
