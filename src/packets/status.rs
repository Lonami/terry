use crate::packets::{PacketBody, NetString};
use crate::serialization::SliceCursor;

/// Ever-increasing server status.
///
/// Direction: Server to Client.
#[derive(Debug)]
pub struct Status {
    pub id: u32,
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
