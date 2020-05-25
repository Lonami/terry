use crate::packets::PacketBody;
use crate::structures::{NetString, RGB, Vec2};
use crate::SliceCursor;

/// Combat text string.
///
/// Direction: Client <-> Server.
#[derive(Debug)]
pub struct CombatText {
    pub pos: Vec2,
    pub color: RGB,
    pub combat_text: NetString,
}

impl PacketBody for CombatText {
    const TAG: u8 = 119;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.pos);
        cursor.write(&self.color);
        cursor.write(&self.combat_text);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            pos: cursor.read(),
            color: cursor.read(),
            combat_text: cursor.read(),
        }
    }
}
