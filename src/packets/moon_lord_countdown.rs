use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Moon Lord Countdown.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct MoonLordCountdown {
    pub moon_lord_countdown: i32,
}

impl PacketBody for MoonLordCountdown {
    const TAG: u8 = 103;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.moon_lord_countdown);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            moon_lord_countdown: cursor.read(),
        }
    }
}
