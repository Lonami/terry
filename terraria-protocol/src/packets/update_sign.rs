use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Update Sign.
///
/// Direction: Updates sign if sent from client otherwise displays sign to client..
#[derive(Debug)]
pub struct UpdateSign {
    pub sign_id: i16,
    pub x: i16,
    pub y: i16,
    pub text: String,
    pub player_id: u8,
    /// BitFlags: 1 = TBD
    pub signflags: u8,
}

impl PacketBody for UpdateSign {
    const TAG: u8 = 47;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.sign_id);
        cursor.write(&self.x);
        cursor.write(&self.y);
        cursor.write(&self.text);
        cursor.write(&self.player_id);
        cursor.write(&self.signflags);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            sign_id: cursor.read(),
            x: cursor.read(),
            y: cursor.read(),
            text: cursor.read(),
            player_id: cursor.read(),
            signflags: cursor.read(),
        }
    }
}
