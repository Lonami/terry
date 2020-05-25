use crate::packets::PacketBody;
use crate::structures::{NetString, RGB};
use crate::SliceCursor;

/// Combat text string.
///
/// Direction: Client <-> Server.
#[derive(Debug)]
pub struct CombatText {
    pub x: i32, /* single */
    pub y: i32, /* single */
    pub color: RGB,
    pub combat_text: NetString,
}

impl PacketBody for CombatText {
    const TAG: u8 = 119;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.x);
        cursor.write(&self.y);
        cursor.write(&self.color);
        cursor.write(&self.combat_text);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            x: cursor.read(),
            y: cursor.read(),
            color: cursor.read(),
            combat_text: cursor.read(),
        }
    }
}
