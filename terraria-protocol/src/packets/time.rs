use crate::packets::PacketBody;
use crate::SliceCursor;

/// Time.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct Time {
    pub daytime: bool,
    pub timevalue: i32,
    pub sunmody: i16,
    pub moonmody: i16,
}

impl PacketBody for Time {
    const TAG: u8 = 18;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.daytime);
        cursor.write(&self.timevalue);
        cursor.write(&self.sunmody);
        cursor.write(&self.moonmody);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            daytime: cursor.read(),
            timevalue: cursor.read(),
            sunmody: cursor.read(),
            moonmody: cursor.read(),
        }
    }
}
