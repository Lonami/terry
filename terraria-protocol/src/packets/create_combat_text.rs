use crate::packets::PacketBody;
use crate::structures::{Vec2, RGB};
use crate::SliceCursor;

/// Create combat text.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct CreateCombatText {
    pub pos: Vec2,
    pub color: RGB,
    pub heal_amount: i32,
}

impl PacketBody for CreateCombatText {
    const TAG: u8 = 81;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.pos);
        cursor.write(&self.color);
        cursor.write(&self.heal_amount);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            pos: cursor.read(),
            color: cursor.read(),
            heal_amount: cursor.read(),
        }
    }
}
