use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// CrystalInvasionSendWaitTime.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct CrystalInvasionSendWaitTime {
    /// 1800 (30s) between waves, 30 (5s) when starting
    pub time_until_next_wave: i32,
}

impl PacketBody for CrystalInvasionSendWaitTime {
    const TAG: u8 = 116;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.time_until_next_wave);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            time_until_next_wave: cursor.read(),
        }
    }
}
