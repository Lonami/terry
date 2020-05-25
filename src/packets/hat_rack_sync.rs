use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Hat rack item sync.
///
/// Direction: Client <-> Server.
#[derive(Debug)]
pub struct HatRackSync {
    pub player_id: u8,
    pub tileentityid: i32,
    pub item_index: u8,
    pub item_id: u16,
    pub stack: u16,
    pub prefix: u8,
}

impl PacketBody for HatRackSync {
    const TAG: u8 = 124;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.tileentityid);
        cursor.write(&self.item_index);
        cursor.write(&self.item_id);
        cursor.write(&self.stack);
        cursor.write(&self.prefix);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            tileentityid: cursor.read(),
            item_index: cursor.read(),
            item_id: cursor.read(),
            stack: cursor.read(),
            prefix: cursor.read(),
        }
    }
}
