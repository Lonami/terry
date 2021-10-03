use crate::serde::{packet_struct, serializable_bitflags};

serializable_bitflags! {
    pub struct ToolMode: u8 {
        const RED = 0x01;
        const GREEN = 0x02;
        const BLUE = 0x04;
        const YELLOW = 0x08;
        const ACTUATOR = 0x16;
        const CUTTER = 0x32;
    }
}

packet_struct! {
    /// Perform a wire operation en-mass.
    ///
    /// Direction: Client -> Server.
    pub struct MassWire {
        const TAG = 109;

        pub start_x: i16,
        pub start_y: i16,
        pub end_x: i16,
        pub end_y: i16,
        pub tool_mode: ToolMode,
    }
}
