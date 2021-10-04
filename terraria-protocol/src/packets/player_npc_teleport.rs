use crate::serde::{fixup_flags, serializable_bitflags, PacketBody, Result, SliceCursor};
use crate::structures::Vec2;

serializable_bitflags! {
    pub struct TeleportMode: u8 {
        const PLAYER = 0x00;
        const NPC = 0x01;
        const PLAYER_TO_PLAYER = 0x02;
        const GET_POS_FROM_TARGET = 0x04;
        const HAS_EXTRA_INFO = 0x08;
    }
}

/// No description known yet.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct PlayerNpcTeleport {
    pub mode: TeleportMode,
    pub target_id: i16,
    pub pos: Vec2,
    pub style: u8,
    /// Only sent if HasExtraInfo flag is true
    pub extra: Option<i32>,
}

impl PacketBody for PlayerNpcTeleport {
    const TAG: u8 = 65;

    fn write_body(&self, cursor: &mut SliceCursor) -> Result<()> {
        cursor.write(&fixup_flags!(TeleportMode where {
            self.extra.is_some() => HAS_EXTRA_INFO,
        } in self.mode))?;
        cursor.write(&self.target_id)?;
        cursor.write(&self.pos)?;
        cursor.write(&self.style)?;
        if let Some(extra) = self.extra {
            cursor.write(&extra)?;
        }
        Ok(())
    }

    fn from_body(cursor: &mut SliceCursor) -> Result<Self> {
        let mode = cursor.read()?;
        Ok(Self {
            mode,
            target_id: cursor.read()?,
            pos: cursor.read()?,
            style: cursor.read()?,
            extra: mode
                .contains(TeleportMode::HAS_EXTRA_INFO)
                .then(|| cursor.read())
                .transpose()?,
        })
    }
}
