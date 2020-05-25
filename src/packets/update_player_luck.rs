use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Update player luck factors.
///
/// Direction: Client <-> Server.
#[derive(Debug)]
pub struct UpdatePlayerLuck {
    pub player_id: u8,
    pub ladybug_luck_time_remaining: i32,
    pub torch_luck: i32 /* single */ ,
    pub luck_potion: u8,
    pub hasgardengnomenearby: bool,
}

impl PacketBody for UpdatePlayerLuck {
    const TAG: u8 = 134;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.ladybug_luck_time_remaining);
        cursor.write(&self.torch_luck);
        cursor.write(&self.luck_potion);
        cursor.write(&self.hasgardengnomenearby);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            ladybug_luck_time_remaining: cursor.read(),
            torch_luck: cursor.read(),
            luck_potion: cursor.read(),
            hasgardengnomenearby: cursor.read(),
        }
    }
}
