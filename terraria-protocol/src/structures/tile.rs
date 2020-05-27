use crate::{Deserializable, Serializable, SliceCursor};

/// Tiles for which the frame is considered "important".
const TILE_FRAME_IMPORTANT: [bool; 623] = [
    false, false, false, true, true, true, false, false, false, false, true, true, true, true,
    true, true, true, true, true, true, true, true, false, false, true, false, true, true, true,
    true, false, true, false, true, true, true, true, false, false, false, false, false, true,
    false, false, false, false, false, false, true, true, false, false, false, false, true, false,
    false, false, false, false, true, false, false, false, false, false, false, false, false,
    false, true, true, true, true, false, false, true, true, true, false, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, false, false, false, true, false, false, true, true, false,
    false, false, false, false, false, false, false, false, false, true, true, false, true, true,
    false, false, true, true, true, true, true, true, true, true, false, true, true, true, true,
    false, false, false, false, true, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, true, false, false, false, false, false, true,
    true, true, true, false, false, false, true, false, false, false, false, false, true, true,
    true, true, false, false, false, false, false, false, false, false, false, false, false, false,
    false, true, false, false, false, false, false, true, false, true, true, false, true, false,
    false, true, true, true, true, true, true, false, false, false, false, false, false, true,
    true, false, false, true, false, true, false, true, true, true, true, true, true, true, true,
    true, true, true, true, true, false, false, false, false, false, false, true, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, true, true,
    true, false, false, false, true, true, true, true, true, true, true, true, true, false, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, false, false, false, true, false, true,
    true, true, true, true, false, false, true, true, false, false, false, false, false, false,
    false, false, false, true, true, false, true, true, true, false, false, false, false, false,
    false, false, false, false, true, false, false, false, false, true, true, true, false, true,
    true, true, true, true, true, true, false, false, false, false, false, false, false, true,
    true, true, true, true, true, true, false, true, false, false, false, false, false, true, true,
    true, true, true, true, true, true, true, true, false, false, false, false, false, false,
    false, false, false, true, true, false, false, false, true, true, true, true, true, false,
    false, false, false, true, true, false, false, true, true, true, false, true, true, true,
    false, false, false, false, false, false, false, false, false, false, true, true, true, true,
    true, true, false, false, false, false, false, false, true, true, true, true, true, true,
    false, false, false, true, true, true, true, true, true, true, true, true, true, true, false,
    false, false, true, true, false, false, false, true, false, false, false, true, true, true,
    true, true, true, true, true, false, true, true, false, false, true, false, true, false, false,
    false, false, false, true, true, false, false, true, true, true, false, false, false, false,
    false, false, true, true, true, true, true, true, true, true, true, true, false, true, true,
    true, true, true, false, false, false, false, true, false, false, false, true, true, true,
    true, false, true, true, true, true, true, true, true, true, true, true, false, true, true,
    true, false, false, false, true, true, false, true, true, true, true, true, true, true, false,
    false, false, false, false, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, false, true, true,
    true, true,
];

#[derive(Debug, Clone)]
pub enum Liquid {
    Water,
    Honey,
    Lava,
}

#[derive(Debug, Default, Clone)]
pub struct Tile {
    pub ty: Option<u16>,
    pub frame: Option<(u16, u16)>,
    pub tile_color: Option<u8>,
    /// If present, `(wall tile id, optional wall color)`.
    pub wall: Option<(u16, Option<u8>)>,
    pub liquid: Option<Liquid>,
    pub wire: [bool; 4],
    pub half_brick: bool,
    pub slope: u8,
    pub actuator: bool,
    pub inactive: bool,
    /// Used only during decoding, to know how many times this tile is
    /// repeated right after.
    pub rle: u16,
}

impl Tile {
    pub fn is_important(&self) -> bool {
        TILE_FRAME_IMPORTANT[self.ty.unwrap_or(0) as usize]
    }
}

impl Serializable for Tile {
    fn serialize(&self, _cursor: &mut SliceCursor) {
        todo!()
    }
}

impl Deserializable for Tile {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        let mut flags: [u8; 3] = [cursor.read(), 0, 0];
        if flags[0] & 0x01 != 0 {
            flags[1] = cursor.read();
            if flags[1] & 0x01 != 0 {
                flags[2] = cursor.read();
            }
        }
        let flags = flags;

        let ty;
        let frame;
        let tile_color;
        if flags[0] & 0x02 != 0 {
            let ty_val = if flags[0] & 0x20 != 0 {
                cursor.read::<u16>()
            } else {
                cursor.read::<u8>() as u16
            };
            ty = Some(ty_val);

            frame = if TILE_FRAME_IMPORTANT[ty_val as usize] {
                Some((cursor.read::<u16>(), cursor.read::<u16>()))
            } else {
                None
            };

            tile_color = if flags[2] & 0x08 != 0 {
                Some(cursor.read::<u8>())
            } else {
                None
            };
        } else {
            ty = None;
            frame = None;
            tile_color = None;
        }

        let mut wall = if flags[0] & 0x04 != 0 {
            let wall = cursor.read::<u8>() as u16;
            if flags[2] & 0x10 != 0 {
                Some((wall, Some(cursor.read::<u8>())))
            } else {
                Some((wall, None))
            }
        } else {
            None
        };

        let liquid = match (flags[0] & 0x18) >> 3 {
            1 => Some(Liquid::Water),
            2 => Some(Liquid::Lava),
            3 => Some(Liquid::Honey),
            _ => None,
        };

        let wire = [
            flags[1] & 0x02 != 0,
            flags[1] & 0x04 != 0,
            flags[1] & 0x08 != 0,
            flags[2] & 0x20 != 0,
        ];

        let shape = flags[1] & 0x70 >> 4;
        let half_brick = shape == 1;
        let slope = if shape > 1 { shape - 1 } else { 0 };

        let actuator = flags[2] & 0x02 != 0;
        let inactive = flags[2] & 0x04 != 0;
        if flags[2] & 0x40 != 0 {
            // this flag basically sets the higher byte of the u16
            wall.as_mut().expect("wall should be present").0 |= (cursor.read::<u8>() as u16) << 8;
        }

        let rle = match (flags[0] & 0xc0) >> 6 {
            0 => 0,
            1 => cursor.read::<u8>() as u16,
            _ => cursor.read::<u16>(),
        };

        Self {
            ty,
            frame,
            tile_color,
            wall,
            liquid,
            wire,
            half_brick,
            slope,
            actuator,
            inactive,
            rle,
        }
    }
}
