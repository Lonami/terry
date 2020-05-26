use crate::{Deserializable, Serializable, SliceCursor};

/// Tiles for which the frame is considered "important".
const TILE_FRAME_IMPORTANT: [bool; 623] = [
    false, false, false, true, true, true, false, false, false, false, true, true, true, true, true, true, true, true, true, true, true, true, false, false, true, false, true, true, true, true, false, true, false, true, true, true, true, false, false, false, false, false, true, false, false, false, false, false, false, true, true, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, true, true, true, false, false, true, true, true, false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, false, false, false, true, false, false, true, true, false, false, false, false, false, false, false, false, false, false, true, true, false, true, true, false, false, true, true, true, true, true, true, true, true, false, true, true, true, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, true, true, true, false, false, false, true, false, false, false, false, false, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, true, true, false, true, false, false, true, true, true, true, true, true, false, false, false, false, false, false, true, true, false, false, true, false, true, false, true, true, true, true, true, true, true, true, true, true, true, true, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, false, false, false, true, true, true, true, true, true, true, true, true, false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, false, false, false, true, false, true, true, true, true, true, false, false, true, true, false, false, false, false, false, false, false, false, false, true, true, false, true, true, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, true, true, false, true, true, true, true, true, true, true, false, false, false, false, false, false, false, true, true, true, true, true, true, true, false, true, false, false, false, false, false, true, true, true, true, true, true, true, true, true, true, false, false, false, false, false, false, false, false, false, true, true, false, false, false, true, true, true, true, true, false, false, false, false, true, true, false, false, true, true, true, false, true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, true, false, false, false, false, false, false, true, true, true, true, true, true, false, false, false, true, true, true, true, true, true, true, true, true, true, true, false, false, false, true, true, false, false, false, true, false, false, false, true, true, true, true, true, true, true, true, false, true, true, false, false, true, false, true, false, false, false, false, false, true, true, false, false, true, true, true, false, false, false, false, false, false, true, true, true, true, true, true, true, true, true, true, false, true, true, true, true, true, false, false, false, false, true, false, false, false, true, true, true, true, false, true, true, true, true, true, true, true, true, true, true, false, true, true, true, false, false, false, true, true, false, true, true, true, true, true, true, true, false, false, false, false, false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, false, true, true, true, true
];

#[derive(Debug)]
pub struct Tile {
    pub wires: [bool; 4],
    pub slopes: [bool; 3],
    pub tile_color: Option<u8>,
    pub wall_color: Option<u8>,
    pub ty: Option<u16>,
    pub frame: Option<(u16, u16)>,
    pub wall: Option<u8>,
    pub liquid: Option<(u8, u8)>,
    pub is_half: bool,
    pub is_actuator: bool,
    pub inactive: bool,
}

impl Serializable for Tile {
    fn serialize(&self, _cursor: &mut SliceCursor) {
        todo!()
    }
}

impl Deserializable for Tile {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        let flags: [u8; 2] = [cursor.read(), cursor.read()];
        let wires: [bool; 4] = [
            flags[0] & 0x10 != 0,
            flags[1] & 0x01 != 0,
            flags[1] & 0x02 != 0,
            false,
        ];
        let slopes: [bool; 3] = [
            flags[1] & 0x10 != 0,
            flags[1] & 0x20 != 0,
            flags[1] & 0x40 != 0,
        ];
        let tile_color = if flags[1] & 0x04 != 0 {
            Some(cursor.read())
        } else {
            None
        }; // u8
        let wall_color = if flags[1] & 0x08 != 0 {
            Some(cursor.read())
        } else {
            None
        }; // u8
        let ty = if flags[0] & 0x01 != 0 {
            Some(cursor.read())
        } else {
            None
        };
        let frame = if let Some(ty) = ty {
            if TILE_FRAME_IMPORTANT[ty as usize - 1] {
                Some((cursor.read(), cursor.read()))
            } else {
                None
            }
        } else {
            None
        };

        let wall = if flags[0] & 0x04 != 0 {
            Some(cursor.read())
        } else {
            None
        };
        let liquid = if flags[0] & 0x08 != 0 {
            Some((cursor.read(), cursor.read()))
        } else {
            None
        };

        let is_half = flags[0] & 0x20 != 0;
        let is_actuator = flags[0] & 0x40 != 0;
        let inactive = flags[0] & 0x80 != 0;

        dbg!(Self {
            wires,
            slopes,
            tile_color,
            wall_color,
            ty,
            frame,
            wall,
            liquid,
            is_half,
            is_actuator,
            inactive,
        })
    }
}
