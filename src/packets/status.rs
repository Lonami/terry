use crate::packets::{NetString, PacketBody};
use crate::serialization::SliceCursor;

/// Status.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct Status {
    /// Status only increases
    pub statusmax: i32,
    pub status_text: NetString,
    pub status_text_flags: u8,
}

impl PacketBody for Status {
    const TAG: u8 = 9;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.statusmax);
        cursor.write(&self.status_text);
        cursor.write(&self.status_text_flags);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            statusmax: cursor.read(),
            status_text: cursor.read(),
            status_text_flags: cursor.read(),
        }
    }
}
