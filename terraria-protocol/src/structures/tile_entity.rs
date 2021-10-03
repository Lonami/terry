use crate::{Deserializable, Serializable, SliceCursor};

#[derive(Debug)]
pub enum TileEntity {
    TrainingDummy {
        id: i32,
        x: i16,
        y: i16,
        npc_index: u16,
    },
    ItemFrame {
        id: i32,
        x: i16,
        y: i16,
        item_type: u16,
        item_prefix: u8,
        item_stack: u16,
    },
    LogicSensor {
        id: i32,
        x: i16,
        y: i16,
        logic_check_type: u8,
        on: bool,
    },
    DisplayDoll {
        id: i32,
        x: i16,
        y: i16,
        flags: [u8; 2], // TODO read body
    },
    WeaponRack {
        id: i32,
        x: i16,
        y: i16,
        item_type: u16,
        item_prefix: u8,
        item_stack: u16,
    },
    HatRack {
        id: i32,
        x: i16,
        y: i16,
        flags: u8, // TODO read body
    },
    FoodPlatter {
        id: i32,
        x: i16,
        y: i16,
        item_type: u16,
        item_prefix: u8,
        item_stack: u16,
    },
    Pylon {
        id: i32,
        x: i16,
        y: i16,
    },
}

impl Serializable for TileEntity {
    fn serialize(&self, _cursor: &mut SliceCursor) {
        todo!()
    }
}

fn read_doll(_cursor: &mut SliceCursor) -> TileEntity {
    todo!()
}

fn read_hat_rack(_cursor: &mut SliceCursor) -> TileEntity {
    todo!()
}

impl Deserializable for TileEntity {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        match cursor.read::<u8>() {
            0 => TileEntity::TrainingDummy {
                id: cursor.read(),
                x: cursor.read(),
                y: cursor.read(),
                npc_index: cursor.read(),
            },
            1 => TileEntity::ItemFrame {
                id: cursor.read(),
                x: cursor.read(),
                y: cursor.read(),
                item_type: cursor.read(),
                item_prefix: cursor.read(),
                item_stack: cursor.read(),
            },
            2 => TileEntity::LogicSensor {
                id: cursor.read(),
                x: cursor.read(),
                y: cursor.read(),
                logic_check_type: cursor.read(),
                on: cursor.read(),
            },
            3 => read_doll(cursor),
            4 => TileEntity::WeaponRack {
                id: cursor.read(),
                x: cursor.read(),
                y: cursor.read(),
                item_type: cursor.read(),
                item_prefix: cursor.read(),
                item_stack: cursor.read(),
            },
            5 => read_hat_rack(cursor),
            6 => TileEntity::FoodPlatter {
                id: cursor.read(),
                x: cursor.read(),
                y: cursor.read(),
                item_type: cursor.read(),
                item_prefix: cursor.read(),
                item_stack: cursor.read(),
            },
            7 => TileEntity::Pylon {
                id: cursor.read(),
                x: cursor.read(),
                y: cursor.read(),
            },
            n => panic!("invalid tile entity {}", n),
        }
    }
}
