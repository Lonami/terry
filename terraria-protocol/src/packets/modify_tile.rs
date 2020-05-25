use crate::packets::PacketBody;
use crate::SliceCursor;

/// Modify a single tile.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct ModifyTile {
    /// Values: 0 = KillTile, 1 = PlaceTile, 2 = KillWall, 3 = PlaceWall, 4 = KillTileNoItem, 5 = PlaceWire, 6 = KillWire, 7 = PoundTile, 8 = PlaceActuator, 9 = KillActuator, 10 = PlaceWire2, 11 = KillWire2, 12 = PlaceWire3, 13 = KillWire3, 14 = SlopeTile, 15 = FrameTrack, 16 = PlaceWire4, 17 = KillWire4, 18 = PokeLogicGate, 19 = Actuate, 20 = KillTile, 21 = ReplaceTile, 22 = ReplaceWall, 23 = SlopePoundTile
    pub action: u8,
    pub tile_x: i16,
    pub tile_y: i16,
    /// KillTile (Fail: Bool), PlaceTile (Type: Byte), KillWall (Fail: Bool), PlaceWall (Type: Byte), KillTileNoItem (Fail: Bool), SlopeTile (Slope: Byte), ReplaceTile (Type: Int16), ReplaceWall (Type: Int16)
    pub flags1: i16,
    /// PlaceTile (Style: Byte), ReplaceTile (Style: Byte)
    pub flags2: u8,
}

impl PacketBody for ModifyTile {
    const TAG: u8 = 17;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.action);
        cursor.write(&self.tile_x);
        cursor.write(&self.tile_y);
        cursor.write(&self.flags1);
        cursor.write(&self.flags2);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            action: cursor.read(),
            tile_x: cursor.read(),
            tile_y: cursor.read(),
            flags1: cursor.read(),
            flags2: cursor.read(),
        }
    }
}
