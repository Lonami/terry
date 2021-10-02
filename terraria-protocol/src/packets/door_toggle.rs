use crate::packets::PacketBody;
use crate::{Deserializable, Serializable, SliceCursor};

#[repr(u8)]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum DoorAction {
    OpenDoor = 0,
    CloseDoor = 1,
    OpenTrapdoor = 2,
    CloseTrapdoor = 3,
    OpenTallGate = 4,
    CloseTallGate = 5,
}

impl Serializable for DoorAction {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write(&(*self as u8));
    }
}

impl Deserializable for DoorAction {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        match cursor.read::<u8>() {
            0 => DoorAction::OpenDoor,
            1 => DoorAction::CloseDoor,
            2 => DoorAction::OpenTrapdoor,
            3 => DoorAction::CloseTrapdoor,
            4 => DoorAction::OpenTallGate,
            5 => DoorAction::CloseTallGate,
            n => panic!("invalid door action {}", n),
        }
    }
}

/// Door Toggle.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct DoorToggle {
    pub action: DoorAction,
    pub tile_x: i16,
    pub tile_y: i16,
    /// When opening a door, a negative direction means to the left, else to the right.
    /// When opening or closing a trapdoor, a positive direction means the player is above.
    pub direction: i8,
}

impl PacketBody for DoorToggle {
    const TAG: u8 = 19;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.action);
        cursor.write(&self.tile_x);
        cursor.write(&self.tile_y);
        cursor.write(&self.direction);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            action: cursor.read(),
            tile_x: cursor.read(),
            tile_y: cursor.read(),
            direction: cursor.read(),
        }
    }
}
