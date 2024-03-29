use crate::serde::{Error, PacketBody, Result, SliceCursor};
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
        cursor.write::<u16>(
            &self
                .message
                .len()
                .try_into()
                .map_err(|_| Error::PrematureEnd)?,
        )?;
        Ok(())
    }

    fn from_body(cursor: &mut SliceCursor) -> Result<Self> {
        let message_color = cursor.read()?;
        let message = cursor.read::<NetString>()?;
        let message_len = cursor.read()?;
        if message.len() as u16 != message_len {
            return Err(Error::MalformedPayload {
                details: format!(
                    "NetString len of {} differs with SmartTextMessage len of {}",
                    message.len(),
                    message_len
                ),
            });
        }
        Ok(Self {
            message_color,
            message,
        })
    }
}
