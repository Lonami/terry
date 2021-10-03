use crate::serde::{PacketBody, Result, SliceCursor};
use crate::structures::{NetString, Rgb};
use std::convert::TryInto;

/// Smart Text Message (FKA. Chat Message v2).
///
/// Direction: Server -> Client.
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SmartTextMessage {
    /// Note that the client cannot change colors
    pub message_color: Rgb,
    pub message: NetString,
}

impl PacketBody for SmartTextMessage {
    const TAG: u8 = 107;

    fn write_body(&self, cursor: &mut SliceCursor) -> Result<()> {
        cursor.write(&self.message_color)?;
        cursor.write(&self.message)?;
        let message_len: u16 = self.message.len().try_into().expect("message too long");
        cursor.write(&message_len)?;
        Ok(())
    }

    fn from_body(cursor: &mut SliceCursor) -> Result<Self> {
        let message_color = cursor.read()?;
        let message = cursor.read::<NetString>()?;
        let message_len = cursor.read()?;
        assert_eq!(message.len() as u16, message_len);
        Ok(Self {
            message_color,
            message,
        })
    }
}
