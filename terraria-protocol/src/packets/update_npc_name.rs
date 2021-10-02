use crate::packets::PacketBody;
use crate::SliceCursor;

/// Update NPC name.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct UpdateNpcName {
    pub npc_id: i16,
    pub name: Option<String>,
    pub town_npc_variation_idx: Option<i32>,
}

impl PacketBody for UpdateNpcName {
    const TAG: u8 = 56;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.npc_id);
        if let Some((name, vi)) = self.name.as_ref().zip(self.town_npc_variation_idx) {
            cursor.write(name);
            cursor.write(&vi);
        }
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        let npc_id = cursor.read();
        let (name, town_npc_variation_idx) = if cursor.eof() {
            (None, None)
        } else {
            (Some(cursor.read()), Some(cursor.read()))
        };

        Self {
            npc_id,
            name,
            town_npc_variation_idx,
        }
    }
}
