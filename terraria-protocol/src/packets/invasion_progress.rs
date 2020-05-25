use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Report the invasion progress.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct InvasionProgress {
    pub progress: i32,
    pub max_progress: i32,
    pub icon: i8,
    pub wave: i8,
}

impl PacketBody for InvasionProgress {
    const TAG: u8 = 78;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.progress);
        cursor.write(&self.max_progress);
        cursor.write(&self.icon);
        cursor.write(&self.wave);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            progress: cursor.read(),
            max_progress: cursor.read(),
            icon: cursor.read(),
            wave: cursor.read(),
        }
    }
}
