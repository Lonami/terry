use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Goes to spawn, either by login, death or recall.
#[derive(Debug)]
pub struct ToSpawn {
    pub x: i16,
    pub y: i16,
    pub timer: u32,
    pub how: u8,
}

impl PacketBody for ToSpawn {
    const TAG: u8 = 12;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.x);
        cursor.write(&self.y);
        cursor.write(&self.timer);
        cursor.write(&self.how);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            x: cursor.read(),
            y: cursor.read(),
            timer: cursor.read(),
            how: cursor.read(),
        }
    }
}
