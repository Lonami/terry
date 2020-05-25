use crate::{Deserializable, Serializable, SliceCursor};

pub enum TileEntity {
    TrainingDummy {
        id: i32,
        x: u16,
        y: u16,
        npc_index: u16,
    },
    ItemFrame {
        id: i32,
        x: u16,
        y: u16,
        item_type: u16,
        item_prefix: u8,
        item_stack: u16,
    },
    LogicSensor {
        id: i32,
        x: u16,
        y: u16,
        logic_check_type: u8,
        on: bool,
    },
    DisplayDoll {
        id: i32,
        x: u16,
        y: u16,
        flags: [u8; 2], // TODO read body
    },
    WeaponRack {
        id: i32,
        x: u16,
        y: u16,
        item_type: u16,
        item_prefix: u8,
        item_stack: u16,
    },
    HatRack {
        id: i32,
        x: u16,
        y: u16,
        flags: u8, // TODO read body
    },
    FloodPlatter {
        id: i32,
        x: u16,
        y: u16,
        item_type: u16,
        item_prefix: u8,
        item_stack: u16,
    },
    Pylon {
        id: i32,
        x: u16,
        y: u16,
    },
}

impl Serializable for TileEntity {
    fn serialize(&self, cursor: &mut SliceCursor) {
        todo!()
    }
}

impl Deserializable for TileEntity {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        todo!()
    }
}
