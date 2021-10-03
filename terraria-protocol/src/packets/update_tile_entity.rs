use crate::serde::{PacketBody, Result, SliceCursor};

/// Update Tile Entity.
///
/// Direction: Server -> Client.
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct UpdateTileEntity {
    pub id: i32,
    /// If it should be removed, indicate the type and ``(x, y)`` coordinates
    pub remove: Option<(u8, i16, i16)>,
}

impl PacketBody for UpdateTileEntity {
    const TAG: u8 = 86;

    fn write_body(&self, cursor: &mut SliceCursor) -> Result<()> {
        cursor.write(&self.id)?;
        cursor.write(&self.remove.is_some())?;
        if let Some((ty, x, y)) = self.remove {
            cursor.write(&ty)?;
            cursor.write(&x)?;
            cursor.write(&y)?;
        }
        Ok(())
    }

    fn from_body(cursor: &mut SliceCursor) -> Result<Self> {
        let id = cursor.read()?;
        let remove = if cursor.read::<bool>()? {
            None
        } else {
            Some((cursor.read()?, cursor.read()?, cursor.read()?))
        };

        Ok(Self { id, remove })
    }
}
