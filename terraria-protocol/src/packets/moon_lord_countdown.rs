use crate::packets::PacketBody;
use crate::SliceCursor;

/// Countdown for Moon Lord.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct MoonLordCountdown {
    pub countdown: i32,
}

impl PacketBody for MoonLordCountdown {
    const TAG: u8 = 103;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.countdown);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            countdown: cursor.read(),
        }
    }
}
