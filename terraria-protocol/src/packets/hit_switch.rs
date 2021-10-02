use crate::packets::packet_struct;

packet_struct! {
    /// Hit a switch.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct HitSwitch {
        const TAG = 59;

        pub x: i16,
        pub y: i16,
    }
}
