use crate::packets::PacketBody;
use crate::SliceCursor;

/// Update NPC Buff.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct UpdateNpcBuff {
    pub npc_id: i16,
    pub buff_id_1: u16,
    pub time_1: i16,
    pub buff_id_2: u16,
    pub time_2: i16,
    pub buff_id_3: u16,
    pub time_3: i16,
    pub buff_id_4: u16,
    pub time_4: i16,
    pub buff_id_5: u16,
    pub time_5: i16,
}

impl PacketBody for UpdateNpcBuff {
    const TAG: u8 = 54;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.npc_id);
        cursor.write(&self.buff_id_1);
        cursor.write(&self.time_1);
        cursor.write(&self.buff_id_2);
        cursor.write(&self.time_2);
        cursor.write(&self.buff_id_3);
        cursor.write(&self.time_3);
        cursor.write(&self.buff_id_4);
        cursor.write(&self.time_4);
        cursor.write(&self.buff_id_5);
        cursor.write(&self.time_5);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            npc_id: cursor.read(),
            buff_id_1: cursor.read(),
            time_1: cursor.read(),
            buff_id_2: cursor.read(),
            time_2: cursor.read(),
            buff_id_3: cursor.read(),
            time_3: cursor.read(),
            buff_id_4: cursor.read(),
            time_4: cursor.read(),
            buff_id_5: cursor.read(),
            time_5: cursor.read(),
        }
    }
}
