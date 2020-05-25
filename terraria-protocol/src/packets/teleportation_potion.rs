use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Teleportation potion.
///
/// Direction: Server <-> Client.
#[derive(Debug)]
pub struct TeleportationPotion {
    /// 0 = Teleportation Potion, 1 = Magic Conch, 2 = Demon Conch
    pub ty: u8,
}

impl PacketBody for TeleportationPotion {
    const TAG: u8 = 73;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.ty);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self { ty: cursor.read() }
    }
}
