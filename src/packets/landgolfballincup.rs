use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// LandGolfBallInCup.
///
/// Direction: Client <-> Server.
#[derive(Debug)]
pub struct LandGolfBallInCup {
    pub player_id: u8,
    pub x: u16,
    pub y: u16,
    pub number_of_hits: u16,
    pub proj_id: u16,
}

impl PacketBody for LandGolfBallInCup {
    const TAG: u8 = 128;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.x);
        cursor.write(&self.y);
        cursor.write(&self.number_of_hits);
        cursor.write(&self.proj_id);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            x: cursor.read(),
            y: cursor.read(),
            number_of_hits: cursor.read(),
            proj_id: cursor.read(),
        }
    }
}
