use crate::serde::{packet_struct, serializable_enum};

serializable_enum! {
    pub enum DoorAction: u8 {
        OpenDoor = 0,
        CloseDoor = 1,
        OpenTrapdoor = 2,
        CloseTrapdoor = 3,
        OpenTallGate = 4,
        CloseTallGate = 5,
    }
}

packet_struct! {
    /// Door Toggle.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct DoorToggle {
        const TAG = 19;

        pub action: DoorAction,
        pub tile_x: i16,
        pub tile_y: i16,
        /// When opening a door, a negative direction means to the left, else to the right.
        /// When opening or closing a trapdoor, a positive direction means the player is above.
        pub direction: i8,
    }
}
