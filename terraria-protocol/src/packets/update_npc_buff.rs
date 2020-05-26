use crate::packets::PacketBody;
use crate::SliceCursor;

/// Update NPC Buff.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct UpdateNpcBuff {
    pub npc_id: i16,
    /// Tuples of ``(buff id, time)``.
    pub buff_times: [(u16, i16); 5],
}

impl PacketBody for UpdateNpcBuff {
    const TAG: u8 = 54;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.npc_id);
        self.buff_times.iter().for_each(|(b, t)| {
            cursor.write(b);
            cursor.write(t);
        })
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        let npc_id = cursor.read();
        let mut buff_times = [(0, 0); 5];
        buff_times
            .iter_mut()
            .for_each(|tuple| *tuple = (cursor.read(), cursor.read()));
        Self { npc_id, buff_times }
    }
}
