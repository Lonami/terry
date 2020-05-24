use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Player buffs and debuffs.
pub struct PlayerBuffs {
    pub buffs: [u16; 22],
}

impl PacketBody for PlayerBuffs {
    const TAG: u8 = 50;

    fn write_body(&self, cursor: &mut SliceCursor) {
        self.buffs.iter().for_each(|buff| cursor.write(buff));
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        let mut buffs = [0; 22];
        buffs.iter_mut().for_each(|buff| *buff = cursor.read());
        Self { buffs }
    }
}
