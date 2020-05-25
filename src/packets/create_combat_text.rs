use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Create Combat Text.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct CreateCombatText {
    pub x: i32 /* single */ ,
    pub y: i32 /* single */ ,
    pub color: Color,
    pub heal_amount: i32,
}

impl PacketBody for CreateCombatText {
    const TAG: u8 = 81;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.x);
        cursor.write(&self.y);
        cursor.write(&self.color);
        cursor.write(&self.heal_amount);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            x: cursor.read(),
            y: cursor.read(),
            color: cursor.read(),
            heal_amount: cursor.read(),
        }
    }
}
