use crate::packets::PacketBody;
use crate::SliceCursor;

/// Consume wires en-mass.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct MassConsumeWire {
    pub item_type: i16,
    pub quantity: i16,
    pub player_id: u8,
}

impl PacketBody for MassConsumeWire {
    const TAG: u8 = 110;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.item_type);
        cursor.write(&self.quantity);
        cursor.write(&self.player_id);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            item_type: cursor.read(),
            quantity: cursor.read(),
            player_id: cursor.read(),
        }
    }
}
