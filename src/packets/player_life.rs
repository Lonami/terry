use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Current and maximum life of the player.
#[derive(Debug)]
pub struct PlayerLife {
    pub id: u8,
    pub life: u16,
    pub max_life: u16,
}

impl PacketBody for PlayerLife {
    const TAG: u8 = 16;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.id);
        cursor.write(&self.life);
        cursor.write(&self.max_life);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            id: cursor.read(),
            life: cursor.read(),
            max_life: cursor.read(),
        }
    }
}
