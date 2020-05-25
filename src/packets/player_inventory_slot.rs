use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Player Inventory Slot.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct PlayerInventorySlot {
    pub player_id: u8,
    /// 0 - 58 = Inventory, 59 - 78 = Armor, 79 - 88 = Dye, 89 - 93 MiscEquips, 94 - 98 = MiscDyes, 99 - 138 = Piggy bank, 139 - 178 = Safe, 179 = Trash, 180 - 219 = Defender's Forge, 220 - 259 = Void Vault
    pub slot_id: i16,
    pub stack: i16,
    pub prefix: u8,
    pub item_netid: i16,
}

impl PacketBody for PlayerInventorySlot {
    const TAG: u8 = 5;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.slot_id);
        cursor.write(&self.stack);
        cursor.write(&self.prefix);
        cursor.write(&self.item_netid);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            slot_id: cursor.read(),
            stack: cursor.read(),
            prefix: cursor.read(),
            item_netid: cursor.read(),
        }
    }
}
