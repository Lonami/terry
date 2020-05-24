use crate::packets::{PacketBody, Vec2};
use crate::serialization::SliceCursor;

/// Magic sent at the very beginning of the communication.
#[derive(Debug)]
pub struct ItemMoved {
    pub created: u16,
    pub pos: Vec2,
    pub vel: Vec2,
    pub count: u16,
    pub modifier: u8,
    pub a: u8,
    pub id: u16,
}

impl PacketBody for ItemMoved {
    const TAG: u8 = 21;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.created);
        cursor.write(&self.pos);
        cursor.write(&self.vel);
        cursor.write(&self.count);
        cursor.write(&self.modifier);
        cursor.write(&self.a);
        cursor.write(&self.id);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            created: cursor.read(),
            pos: cursor.read(),
            vel: cursor.read(),
            count: cursor.read(),
            modifier: cursor.read(),
            a: cursor.read(),
            id: cursor.read(),
        }
    }
}
