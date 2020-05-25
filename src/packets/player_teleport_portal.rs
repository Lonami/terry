use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Player Teleport Portal.
///
/// Direction: Server <-> Client.
#[derive(Debug)]
pub struct PlayerTeleportPortal {
    pub player_id: u8,
    pub portal_color_index: i16,
    pub new_position_x: i32 /* single */ ,
    pub new_position_y: i32 /* single */ ,
    pub velocity_x: i32 /* single */ ,
    pub velocity_y: i32 /* single */ ,
}

impl PacketBody for PlayerTeleportPortal {
    const TAG: u8 = 96;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.portal_color_index);
        cursor.write(&self.new_position_x);
        cursor.write(&self.new_position_y);
        cursor.write(&self.velocity_x);
        cursor.write(&self.velocity_y);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            portal_color_index: cursor.read(),
            new_position_x: cursor.read(),
            new_position_y: cursor.read(),
            velocity_x: cursor.read(),
            velocity_y: cursor.read(),
        }
    }
}
