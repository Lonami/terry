use crate::packets::PacketBody;
use crate::SliceCursor;

/// Send password.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct SendPassword {
    pub password: String,
}

impl PacketBody for SendPassword {
    const TAG: u8 = 38;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.password);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            password: cursor.read(),
        }
    }
}
