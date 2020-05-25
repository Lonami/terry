use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

pub enum SlotLocation {
    Inventory(usize),
    Armor(usize),
    Dye(usize),
    MiscEquips(usize),
    MiscDyes(usize),
    PiggyBank(usize),
    Safe(usize),
    Trash,
    DefenderForge(usize),
    VoidVault(usize),
}

/// Items in the player's inventory.
///
/// Direction: Bidirectional.
#[derive(Debug)]
pub struct PlayerInventory {
    pub player: u8,
    pub slot: u16,
    pub stack: u16,
    pub prefix: u8,
    pub item_id: u16,
}

impl PlayerInventory {
    fn slot_location(&self) -> SlotLocation {
        let index = self.slot as usize;
        match self.slot {
            0..=58 => SlotLocation::Inventory(index),
            59..=78 => SlotLocation::Armor(index - 59),
            79..=88 => SlotLocation::Dye(index - 79),
            89..=93 => SlotLocation::MiscEquips(index - 89),
            94..=98 => SlotLocation::MiscDyes(index - 94),
            99..=138 => SlotLocation::PiggyBank(index - 99),
            139..=178 => SlotLocation::Safe(index - 139),
            179 => SlotLocation::Trash,
            180..=219 => SlotLocation::DefenderForge(index - 180),
            220..=259 => SlotLocation::VoidVault(index - 220),
            n => panic!(format!("slot index {} is out of bounds", n)),
        }
    }
}

impl PacketBody for PlayerInventory {
    const TAG: u8 = 5;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player);
        cursor.write(&self.slot);
        cursor.write(&self.stack);
        cursor.write(&self.prefix);
        cursor.write(&self.item_id);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player: cursor.read(),
            slot: cursor.read(),
            stack: cursor.read(),
            prefix: cursor.read(),
            item_id: cursor.read(),
        }
    }
}
