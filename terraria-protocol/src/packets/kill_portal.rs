use crate::serde::packet_struct;

packet_struct! {
    /// Kill a portal.
    ///
    /// Direction: Client -> Server.
    pub struct KillPortal {
        const TAG = 95;

        pub projectile_owner: u16,
        pub projectile_ai: u8,
    }
}
