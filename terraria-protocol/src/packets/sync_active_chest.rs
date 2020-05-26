use crate::packets::PacketBody;
use crate::SliceCursor;

/// Used to tell the server that you've exited the chest view (sending ID -1),
/// looking at your piggy bank (sending ID -2), looking at your safe (sending
/// ID -3) or looking at your defender's forge (sending ID -4).
///
/// These are sent at every chest interaction.
///
/// The main function of the packet is to synchronize the sending client's
/// active chest to the server and its side function is to rename the chest.
///
/// It should be noted that this packet is not sent when you open a regular
/// chest. The server knows which chest you opened when you send packet 31
/// `OpenChest`, so this one is only sent upon exit to unblock the chest
/// (as opposed to both open & exit for banks like piggy, safe & defender
/// forge).
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct SyncActiveChest {
    pub id: i16,
    pub x: i16,
    pub y: i16,
    pub name: String,
}

impl PacketBody for SyncActiveChest {
    const TAG: u8 = 33;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.id);
        cursor.write(&self.x);
        cursor.write(&self.y);
        cursor.write(&self.name);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            id: cursor.read(),
            x: cursor.read(),
            y: cursor.read(),
            name: cursor.read(),
        }
    }
}
