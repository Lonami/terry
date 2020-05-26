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

    let mut tiles = Vec::new();
    let mut chests = Vec::new();
    let mut signs = Vec::new();
    let mut tile_entities = Vec::new();

    tiles = Vec::with_capacity((width * height) as usize);

    let mut tile = Tile::default();
    let mut rle = 0i16; // kind of a run-length encoding
    for _ in y_start..y_start + height as i32 {
        for _ in x_start..x_start + width as i32 {
            if rle != 0 {
                rle -= 1;
                tiles.push(tile.clone());
            } else {
                tile = Tile::default();

                let mut flags: [u8; 3] = [cursor.read(), 0, 0];
                if flags[0] & 0x01 != 0 {
                    flags[1] = cursor.read();
                    if flags[1] & 0x01 != 0 {
                        flags[2] = cursor.read();
                    }
                }
                let flags = flags;

                if flags[0] & 0x02 != 0 {
                    if flags[0] & 0x20 != 0 {
                        tile.ty = cursor.read::<u16>();
                    } else {
                        tile.ty = cursor.read::<u8>() as u16;
                    }

                    if tile.is_important() {
                        tile.frame = (cursor.read::<i16>(), cursor.read::<i16>());
                    } else {
                        tile.frame = (-1, -1);
                    }

                    if flags[2] & 0x08 != 0 {
                        tile.tile_color = cursor.read::<u8>();
                    }
                }

                if flags[0] & 0x04 != 0 {
                    tile.wall = cursor.read::<u8>() as u16;
                    if flags[2] & 0x10 != 0 {
                        tile.wall_color = cursor.read::<u8>();
                    }
                }

                tile.liquid = match (flags[0] & 0x18) >> 3 {
                    1 => Some(Liquid::Water),
                    2 => Some(Liquid::Lava),
                    3 => Some(Liquid::Honey),
                    _ => None
                };

                tile.wire = [
                    flags[1] & 0x02 != 0,
                    flags[1] & 0x04 != 0,
                    flags[1] & 0x08 != 0,
                    flags[2] & 0x20 != 0,
                ];
                match flags[1] & 0x70 >> 4 {
                    0 => {},
                    1 => tile.half_brick = true,
                    n => tile.slope = n - 1,
                }

                tile.actuator = flags[2] & 0x02 != 0;
                tile.inactive = flags[2] & 0x04 != 0;
                if flags[2] & 0x40 != 0 {
                    tile.wall |= (cursor.read::<u8>() as u16) << 8;
                }

                tiles.push(tile.clone());

                match (flags[0] & 0xc0) >> 6 {
                    0 => rle = 0,
                    1 => rle = cursor.read::<u8>() as i16,
                    _ => rle = cursor.read::<i16>(),
                }
            }
        }
    }

    chests = Vec::with_capacity(cursor.read::<u16>() as usize);
    (0..chests.capacity()).for_each(|_| chests.push(cursor.read()));
    signs = Vec::with_capacity(cursor.read::<u16>() as usize);
    (0..signs.capacity()).for_each(|_| signs.push(cursor.read()));
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
