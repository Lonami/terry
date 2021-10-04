use crate::serde::{Deserializable, PacketBody, Result, Serializable, SliceCursor};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Emote {
    pub player_id: u16,
    pub lifetime: u16,
    pub emote: u8,
    pub metadata: Option<i16>,
}

/// Sync an emote bubble.
///
/// Direction: Server -> Client.
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SyncEmoteBubble {
    pub emote_id: i32,
    pub anchor_type: u8,
    pub emote: Option<Emote>,
}

impl Serializable for Emote {
    fn serialize(&self, cursor: &mut SliceCursor) -> Result<()> {
        cursor.write(&self.player_id)?;
        cursor.write(&self.lifetime)?;
        if let Some(metadata) = self.metadata.as_ref() {
            cursor.write(&(self.emote | 0x80))?;
            cursor.write(metadata)?;
        } else {
            cursor.write(&(self.emote & 0x7f))?;
        }
        Ok(())
    }
}

impl Deserializable for Emote {
    fn deserialize(cursor: &mut SliceCursor) -> Result<Self> {
        let player_id = cursor.read()?;
        let lifetime = cursor.read()?;
        let mut emote = cursor.read()?;
        let metadata = if emote & 0x80 != 0 {
            emote &= 0x7f;
            Some(cursor.read()?)
        } else {
            None
        };

        Ok(Self {
            player_id,
            lifetime,
            emote,
            metadata,
        })
    }
}

impl PacketBody for SyncEmoteBubble {
    const TAG: u8 = 91;

    fn write_body(&self, cursor: &mut SliceCursor) -> Result<()> {
        cursor.write(&self.emote_id)?;
        if let Some(emote) = self.emote.as_ref() {
            cursor.write(&self.anchor_type)?;
            cursor.write(emote)?;
        } else {
            cursor.write(&255u8)?;
        }
        Ok(())
    }

    fn from_body(cursor: &mut SliceCursor) -> Result<Self> {
        let emote_id = cursor.read()?;
        let anchor_type = cursor.read()?;

        let emote = if anchor_type != 255 {
            Some(cursor.read()?)
        } else {
            None
        };

        Ok(Self {
            emote_id,
            anchor_type,
            emote,
        })
    }
}
