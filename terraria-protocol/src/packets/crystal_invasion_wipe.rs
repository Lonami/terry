use crate::packets::packet_struct;

packet_struct! {
    /// Wipe everything in the Eternia Crystal Invasion.
    ///
    /// Direction: Server -> Client.
    pub struct CrystalInvasionWipe {
        const TAG = 114;
    }
}
