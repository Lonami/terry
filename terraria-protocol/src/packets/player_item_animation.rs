use crate::packets::PacketBody;
use crate::SliceCursor;

/// Player item animation.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct PlayerItemAnimation {
    pub player_id: u8,
    pub item_rotation: i32, /* single */
    pub item_animation: i16,
}

impl PacketBody for PlayerItemAnimation {
    const TAG: u8 = 41;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.item_rotation);
        cursor.write(&self.item_animation);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            item_rotation: cursor.read(),
            item_animation: cursor.read(),
        }
    }
}
