use crate::structures::LiquidType;
use crate::{Deserializable, Serializable, SliceCursor};
use bitflags::bitflags;

/// Tiles for which the frame is considered "important".
const TILE_FRAME_IMPORTANT: [bool; 624] = [
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
    false, false, false, false, false, true, true, true, true, true, true, true, true, true, true,
    true, false, false, false, false, false, false, true, true, true, true, true, true, false,
    false, false, true, true, true, true, true, true, true, true, true, true, true, false, false,
    false, true, true, false, false, false, true, false, false, false, true, true, true, true,
    true, true, true, true, false, true, true, false, false, true, false, true, false, false,
    false, false, false, true, true, false, false, true, true, true, false, false, false, false,
    false, false, true, true, true, true, true, true, true, true, true, true, false, true, true,
    true, true, true, false, false, false, false, true, false, false, false, true, true, true,
    true, false, true, true, true, true, true, true, true, true, true, true, false, true, true,
    true, false, false, false, true, true, false, true, true, true, true, true, true, true, false,
    false, false, false, false, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, false, true, true,
    true, true, true,
];

bitflags! {
    pub struct TileFlags: u16 {
        const ACTIVE = 0x0001;
        const LIGHTED = 0x0002;
        const HAS_WALL = 0x0004;
        const HAS_LIQUID = 0x0008;
        const WIRE1 = 0x0010;
        const HALF_BRICK = 0x0020;
        const ACTUATOR = 0x0040;
        const INACTIVE = 0x0080;
        const WIRE2 = 0x0100;
        const WIRE3 = 0x0200;
        const HAS_COLOR = 0x0400;
        const HAS_WALL_COLOR = 0x0800;
        const SLOPE1 = 0x1000;
        const SLOPE2 = 0x2000;
        const SLOPE3 = 0x4000;
        const WIRE4 = 0x8000;
    }
}

#[derive(Debug, Clone)]
pub struct Tile {
    flags: TileFlags,
    color: Option<u8>,
    wall_color: Option<u8>,
    ty: Option<u16>,
    frame: Option<(i16, i16)>,
    wall: Option<u16>,
    liquid: Option<(u8, LiquidType)>,
}

impl Serializable for Tile {
    fn serialize(&self, _cursor: &mut SliceCursor) {
        todo!()
    }
}

impl Deserializable for Tile {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        let flags = TileFlags::from_bits_truncate(cursor.read());

        let color = flags.contains(TileFlags::HAS_COLOR).then(|| cursor.read());

        let wall_color = flags
            .contains(TileFlags::HAS_WALL_COLOR)
            .then(|| cursor.read());

        let ty = flags.contains(TileFlags::ACTIVE).then(|| cursor.read());

        let frame = ty
            .map(|t| TILE_FRAME_IMPORTANT[t as usize])
            .unwrap_or(false)
            .then(|| (cursor.read(), cursor.read()));

        let wall = flags.contains(TileFlags::HAS_WALL).then(|| cursor.read());
        let liquid = flags
            .contains(TileFlags::HAS_LIQUID)
            .then(|| (cursor.read(), cursor.read()));

        Self {
            flags,
            color,
            wall_color,
            ty,
            frame,
            wall,
            liquid,
        }
    }
}

impl Tile {
    pub fn deserialize_packed(cursor: &mut SliceCursor) -> (Self, u16) {
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
        let color;
        if flags[0] & 0x02 != 0 {
            let ty_val = if flags[0] & 0x20 != 0 {
                cursor.read::<u16>()
            } else {
                cursor.read::<u8>() as u16
            };
            ty = Some(ty_val);

            frame = if TILE_FRAME_IMPORTANT[ty_val as usize] {
                Some((cursor.read(), cursor.read()))
            } else {
                None
            };

            color = if flags[2] & 0x08 != 0 {
                Some(cursor.read::<u8>())
            } else {
                None
            };
        } else {
            ty = None;
            frame = None;
            color = None;
        }

        let mut wall_color = None;
        let mut wall = if flags[0] & 0x04 != 0 {
            let wall = cursor.read::<u8>() as u16;
            if flags[2] & 0x10 != 0 {
                wall_color = Some(cursor.read::<u8>());
            }
            Some(wall)
        } else {
            None
        };

        let liquid = match (flags[0] & 0x18) >> 3 {
            1 => Some((cursor.read(), LiquidType::Water)),
            2 => Some((cursor.read(), LiquidType::Lava)),
            3 => Some((cursor.read(), LiquidType::Honey)),
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
            *wall.as_mut().expect("wall should be present") |= (cursor.read::<u8>() as u16) << 8;
        }

        let rle = match (flags[0] & 0xc0) >> 6 {
            0 => 0,
            1 => cursor.read::<u8>() as u16,
            _ => cursor.read::<u16>(),
        };

        (
            Self {
                flags: TileFlags::empty(),
                color,
                wall_color,
                ty,
                frame,
                wall,
                liquid,
            },
            rle,
        )
    }
}
