use crate::packets::{PacketBody, Vec2};
use crate::serialization::SliceCursor;

/// Movement of a player, optionally including speed.
#[derive(Debug)]
pub struct PlayerMove {
    pub id: u8,
    pub flags: u8,
    pub speed_flags: u8,
    pub c: u8, // ???
    pub d: u8, // ???
    pub hotbar: u8,
    pub pos: Vec2,
    pub vel: Option<Vec2>,
}

impl PacketBody for PlayerMove {
    const TAG: u8 = 13;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.id);
        cursor.write(&self.flags);
        cursor.write(&self.speed_flags);
        cursor.write(&self.c);
        cursor.write(&self.d);
        cursor.write(&self.hotbar);
        cursor.write(&self.pos);
        if let Some(vel) = self.vel.as_ref() {
            cursor.write(vel);
        }
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        let id = cursor.read();
        let flags = cursor.read();
        let speed_flags = cursor.read();
        let c = cursor.read();
        let d = cursor.read();
        let hotbar = cursor.read();
        let pos = cursor.read();
        let vel = if speed_flags & 0x04 != 0 {
            Some(cursor.read())
        } else {
            None
        };

        Self {
            id,
            flags,
            speed_flags,
            c,
            d,
            hotbar,
            pos,
            vel,
        }
    }
}
