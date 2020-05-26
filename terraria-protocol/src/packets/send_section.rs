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

    let mut tiles = Vec::new();
    let mut chests = Vec::new();
    let mut signs = Vec::new();
    let mut tile_entities = Vec::new();

    tiles = Vec::with_capacity((width * height) as usize);

    let mut tile = Tile::default();
    let mut num1 = 0;
    for index1 in y_start..y_start + height as i32 {
        for index2 in x_start..x_start + width as i32 {
            if num1 != 0 {
                num1 -= 1;
                tiles.push(tile.clone());
            } else {
                let mut num2 = 0u8;
                let mut num3 = 0u8;
                tile = Tile::default();
                let num4 = cursor.read::<u8>();
                if num4 & 0x01 != 0 {
                    num3 = cursor.read();
                    if num3 & 0x01 != 0 {
                        num2 = cursor.read();
                    }
                }

                let flag = true; // tile active
                if num4 & 0x02 != 0 {
                    // set tile active true
                    let ty = 0; // tile ty
                    let mut index3 = 0i32;
                    if num4 & 0x20 != 0 {
                        let num5 = cursor.read::<u8>();
                        index3 = ((cursor.read::<u8>() as i32) << 8) | (num5 as i32);
                    } else {
                        index3 = cursor.read::<u8>() as i32;
                    }

                    ty = index3 as u16;
                    if tile_important(index3) {
                        let frame_x = cursor.read::<i16>();
                        let frame_y = cursor.read::<i16>();
                    } else {
                        let frame_x = -1i16;
                        let frame_y = -1i16;
                    }

                    if num2 & 0x08 != 0 {
                        let color = cursor.read::<u8>();
                    }
                }

                if num4 & 0x04 != 0 {
                    let wall = cursor.read::<u8>() as u16;
                    if num2 & 0x10 != 0 {
                        let wall_color = cursor.read::<u8>();
                    }
                }

                let num6 = (num4 & 24) >> 3;
                if num6 != 0 {
                    let liquid = cursor.read::<u8>();
                    if num6 > 1 {
                        if num6 == 2 {
                            let lava = true;
                        } else {
                            let honey = true;
                        }
                    }
                }

                if num3 > 1 {
                    if num3 & 0x02 != 0 {
                        let wire = true;
                    }
                    if num3 & 0x04 != 0 {
                        let wire2 = true;
                    }
                    if num3 & 0x08 != 0 {
                        let wire3 = true;
                    }
                    let num5 = (num3 & 112) >> 4;
                    if num5 != 0 && tile_solid(ty) {
                        if num5 == 1 {
                            let half_brick = true;
                        } else {
                            let slope = num5 - 1;
                        }
                    }
                }

                if num2 > 0 {
                    if num2 & 0x02 != 0 {
                        let actuator = true;
                    }
                    if num2 & 0x04 != 0 {
                        let inactive = true;
                    }
                    if num2 & 0x20 != 0 {
                        let wire4 = true;
                    }
                    if num2 & 0x40 != 0 {
                        let num5 = cursor.read::<u8>();
                        let wall = ((num5 as u16) << 8) | wall;
                    }
                }

                match (num4 & 192) >> 6 {
                    0 => num1 = 0,
                    1 => num1 = cursor.read::<u8>(),
                    _ => num1 = cursor.read::<i16>(),
                }
            }
        }
    }

    (0..tiles.capacity()).for_each(|_| tiles.push(dbg!(cursor.read())));
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
