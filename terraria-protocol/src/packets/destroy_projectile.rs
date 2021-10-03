use crate::serde::packet_struct;

packet_struct! {
    /// Destroy a projectile.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct DestroyProjectile {
        const TAG = 29;

        pub projectile_id: i16,
        /// Player ID
        pub owner: u8,
    }
}
