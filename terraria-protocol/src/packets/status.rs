use crate::packets::PacketBody;
use crate::structures::NetString;
use crate::SliceCursor;

/// Status.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct Status {
    /// Ever-increasing status identifier
    pub id: i32,
    pub text: NetString,
    pub flags: u8,
}

impl PacketBody for Status {
    const TAG: u8 = 9;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.id);
        cursor.write(&self.text);
        cursor.write(&self.flags);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            id: cursor.read(),
            text: cursor.read(),
            flags: cursor.read(),
        }
    }
}
