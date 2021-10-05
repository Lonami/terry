use crate::serde::{PacketBody, Result, SliceCursor};

/// Superseded by UpdateItemDrop2.
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct UpdateItemDrop(pub super::UpdateItemDrop2);

impl PacketBody for UpdateItemDrop {
    const TAG: u8 = 21;

    fn write_body(&self, cursor: &mut SliceCursor) -> Result<()> {
        self.0.write_body(cursor)
    }

    fn from_body(cursor: &mut SliceCursor) -> Result<Self> {
        super::UpdateItemDrop2::from_body(cursor).map(Self)
    }
}
