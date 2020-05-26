use crate::packets::PacketBody;
use crate::SliceCursor;

/// Time.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct Time {
    pub day_time: bool,
    pub time: i32,
    pub sun_mod_y: i16,
    pub moon_mod_y: i16,
}

impl PacketBody for Time {
    const TAG: u8 = 18;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.day_time);
        cursor.write(&self.time);
        cursor.write(&self.sun_mod_y);
        cursor.write(&self.moon_mod_y);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            day_time: cursor.read(),
            time: cursor.read(),
            sun_mod_y: cursor.read(),
            moon_mod_y: cursor.read(),
        }
    }
}
