use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Player buffs and debuffs.
#[derive(Debug)]
pub struct PlayerBuffs {
    pub id: u8,
    pub buffs: [u16; 22],
}

impl PacketBody for PlayerBuffs {
    const TAG: u8 = 50;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.id);
        self.buffs.iter().for_each(|buff| cursor.write(buff));
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        let id = cursor.read();
        let mut buffs = [0; 22];
        buffs.iter_mut().for_each(|buff| *buff = cursor.read());
        Self { id, buffs }
    }
}
