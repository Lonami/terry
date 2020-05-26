use crate::packets::PacketBody;
use crate::{Deserializable, Serializable, SliceCursor};

#[repr(u8)]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum ChestAction {
    PlaceChest = 0,
    KillChest = 1,
    PlaceDresser = 2,
    KillDresser = 3,
    PlaceContainers = 4,
    KillContainers = 5,
}

impl Serializable for ChestAction {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write(&(*self as u8));
    }
}

impl Deserializable for ChestAction {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        match cursor.read::<u8>() {
            0 => ChestAction::PlaceChest,
            1 => ChestAction::KillChest,
            2 => ChestAction::PlaceDresser,
            3 => ChestAction::KillDresser,
            4 => ChestAction::PlaceContainers,
            5 => ChestAction::KillContainers,
            n => panic!(format!("invalid chest action {}", n)),
        }
    }
}

/// Place a chest.
///
/// Direction: Server <-> Client.
#[derive(Debug)]
pub struct PlaceChest {
    pub action: ChestAction,
    pub tile_x: i16,
    pub tile_y: i16,
    /// FrameX (Chest type)
    pub style: i16,
    /// ID if client is receiving packet, else 0.
    pub chest_id_to_destroy: i16,
}

impl PacketBody for PlaceChest {
    const TAG: u8 = 34;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.action);
        cursor.write(&self.tile_x);
        cursor.write(&self.tile_y);
        cursor.write(&self.style);
        cursor.write(&self.chest_id_to_destroy);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            action: cursor.read(),
            tile_x: cursor.read(),
            tile_y: cursor.read(),
            style: cursor.read(),
            chest_id_to_destroy: cursor.read(),
        }
    }
}
