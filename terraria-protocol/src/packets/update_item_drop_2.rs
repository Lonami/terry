use crate::packets::PacketBody;
use crate::structures::Vec2;
use crate::SliceCursor;

/// Update Item Drop 2.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct UpdateItemDrop2 {
    /// Should set to null ``if item_id < 400 && item_net_id == 0`.
    /// Otherwise, ``if item_id = 400`` it's a new item.
    pub item_id: i16,
    pub pos: Vec2,
    pub vel: Vec2,
    pub stack_size: i16,
    pub prefix: u8,
    /// If 0, set ``own_ignore = 0`` and ``own_time = 100``.
    pub no_delay: u8,
    pub item_net_id: i16,
}

impl PacketBody for UpdateItemDrop2 {
    const TAG: u8 = 90;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.item_id);
        cursor.write(&self.pos);
        cursor.write(&self.vel);
        cursor.write(&self.stack_size);
        cursor.write(&self.prefix);
        cursor.write(&self.no_delay);
        cursor.write(&self.item_net_id);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            item_id: cursor.read(),
            pos: cursor.read(),
            vel: cursor.read(),
            stack_size: cursor.read(),
            prefix: cursor.read(),
            no_delay: cursor.read(),
            item_net_id: cursor.read(),
        }
    }
}
