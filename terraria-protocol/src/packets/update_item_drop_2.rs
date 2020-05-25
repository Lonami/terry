use crate::packets::PacketBody;
use crate::SliceCursor;

/// Update Item Drop 2.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct UpdateItemDrop2 {
    /// If below 400 and NetID 0 Then Set NullIf ItemID is 400 Then New Item
    pub item_id: i16,
    pub position_x: i32, /* single */
    pub position_y: i32, /* single */
    pub velocity_x: i32, /* single */
    pub velocity_y: i32, /* single */
    pub stack_size: i16,
    pub prefix: u8,
    /// If 0 then ownIgnore = 0 and ownTime = 100
    pub nodelay: u8,
    pub item_net_id: i16,
}

impl PacketBody for UpdateItemDrop2 {
    const TAG: u8 = 90;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.item_id);
        cursor.write(&self.position_x);
        cursor.write(&self.position_y);
        cursor.write(&self.velocity_x);
        cursor.write(&self.velocity_y);
        cursor.write(&self.stack_size);
        cursor.write(&self.prefix);
        cursor.write(&self.nodelay);
        cursor.write(&self.item_net_id);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            item_id: cursor.read(),
            position_x: cursor.read(),
            position_y: cursor.read(),
            velocity_x: cursor.read(),
            velocity_y: cursor.read(),
            stack_size: cursor.read(),
            prefix: cursor.read(),
            nodelay: cursor.read(),
            item_net_id: cursor.read(),
        }
    }
}
