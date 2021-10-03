use crate::serde::{PacketBody, SliceCursor};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum TileAction {
    KillTile { fail: bool },
    PlaceTile { ty: u8, style: u8 },
    KillWall { fail: bool },
    PlaceWall { ty: u8 },
    KillTileNoItem { fail: bool },
    PlaceWire,
    KillWire,
    PoundTile,
    PlaceActuator,
    KillActuator,
    PlaceWire2,
    KillWire2,
    PlaceWire3,
    KillWire3,
    SlopeTile { slope: u8 },
    FrameTrack,
    PlaceWire4,
    KillWire4,
    PokeLogicGate,
    Actuate,
    KillTile2 { fail: bool },
    ReplaceTile { ty: i16, style: u8 },
    ReplaceWall { ty: i16 },
    SlopePoundTile,
}

impl Default for TileAction {
    fn default() -> Self {
        Self::KillTile { fail: false }
    }
}

/// Modify a single tile.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ModifyTile {
    pub action: TileAction,
    pub tile_x: i16,
    pub tile_y: i16,
}

impl PacketBody for ModifyTile {
    const TAG: u8 = 17;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write::<u8>(&match self.action {
            TileAction::KillTile { .. } => 0,
            TileAction::PlaceTile { .. } => 1,
            TileAction::KillWall { .. } => 2,
            TileAction::PlaceWall { .. } => 3,
            TileAction::KillTileNoItem { .. } => 4,
            TileAction::PlaceWire => 5,
            TileAction::KillWire => 6,
            TileAction::PoundTile => 7,
            TileAction::PlaceActuator => 8,
            TileAction::KillActuator => 9,
            TileAction::PlaceWire2 => 10,
            TileAction::KillWire2 => 11,
            TileAction::PlaceWire3 => 12,
            TileAction::KillWire3 => 13,
            TileAction::SlopeTile { .. } => 14,
            TileAction::FrameTrack => 15,
            TileAction::PlaceWire4 => 16,
            TileAction::KillWire4 => 17,
            TileAction::PokeLogicGate => 18,
            TileAction::Actuate => 19,
            TileAction::KillTile2 { .. } => 20,
            TileAction::ReplaceTile { .. } => 21,
            TileAction::ReplaceWall { .. } => 22,
            TileAction::SlopePoundTile => 23,
        });
        cursor.write(&self.tile_x);
        cursor.write(&self.tile_y);
        cursor.write(&match self.action {
            TileAction::KillTile { fail } => fail as i16,
            TileAction::PlaceTile { ty, .. } => ty as i16,
            TileAction::KillWall { fail } => fail as i16,
            TileAction::PlaceWall { ty } => ty as i16,
            TileAction::KillTileNoItem { fail } => fail as i16,
            TileAction::SlopeTile { slope } => slope as i16,
            TileAction::KillTile2 { fail } => fail as i16,
            TileAction::ReplaceTile { ty, .. } => ty,
            TileAction::ReplaceWall { ty } => ty,
            _ => 0i16,
        });
        cursor.write(&match self.action {
            TileAction::PlaceTile { style, .. } => style,
            TileAction::ReplaceTile { style, .. } => style,
            _ => 0u8,
        });
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        let action = cursor.read::<u8>();
        let tile_x = cursor.read();
        let tile_y = cursor.read();
        let extra = cursor.read::<i16>();
        let style = cursor.read::<u8>();
        Self {
            action: match action {
                0 => TileAction::KillTile { fail: extra != 0 },
                1 => TileAction::PlaceTile {
                    ty: extra as u8,
                    style,
                },
                2 => TileAction::KillWall { fail: extra != 0 },
                3 => TileAction::PlaceWall { ty: extra as u8 },
                4 => TileAction::KillTileNoItem { fail: extra != 0 },
                5 => TileAction::PlaceWire,
                6 => TileAction::KillWire,
                7 => TileAction::PoundTile,
                8 => TileAction::PlaceActuator,
                9 => TileAction::KillActuator,
                10 => TileAction::PlaceWire2,
                11 => TileAction::KillWire2,
                12 => TileAction::PlaceWire3,
                13 => TileAction::KillWire3,
                14 => TileAction::SlopeTile { slope: extra as u8 },
                15 => TileAction::FrameTrack,
                16 => TileAction::PlaceWire4,
                17 => TileAction::KillWire4,
                18 => TileAction::PokeLogicGate,
                19 => TileAction::Actuate,
                20 => TileAction::KillTile2 { fail: extra != 0 },
                21 => TileAction::ReplaceTile { ty: extra, style },
                22 => TileAction::ReplaceWall { ty: extra },
                23 => TileAction::SlopePoundTile,
                n => panic!("invalid TileAction: {}", n),
            },
            tile_x,
            tile_y,
        }
    }
}
