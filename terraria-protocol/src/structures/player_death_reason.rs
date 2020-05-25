use crate::{Deserializable, Serializable, SliceCursor};

#[derive(Debug)]
pub struct PlayerDeathReason {}

impl Serializable for PlayerDeathReason {
    fn serialize(&self, _cursor: &mut SliceCursor) {
        todo!()
    }
}

impl Deserializable for PlayerDeathReason {
    fn deserialize(_cursor: &mut SliceCursor) -> Self {
        todo!()
    }
}
