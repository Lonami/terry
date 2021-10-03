use crate::packets::packet_struct;

packet_struct! {
    /// Update sign if sent from client otherwise displays sign to client.
    ///
    /// Direction: Client <-> Server.
    pub struct UpdateSign {
        const TAG = 47;

        pub sign_id: i16,
        pub x: i16,
        pub y: i16,
        pub text: String,
        pub player_id: u8,
        pub flags: u8,
    }
}
