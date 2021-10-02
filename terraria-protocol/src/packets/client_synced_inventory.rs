use crate::packets::packet_struct;

packet_struct! {
    /// Client finished inventory changes on this tick.
    ///
    /// It's sent by the client code twice when a player moves an item around in
    /// their inventory, although the packet has no data.
    ///
    /// The total payload size is 2 packets per inventory item drag, with 3 bytes
    /// each (2 for length, 1 for packet ID). This is a functionally useless
    /// packet.
    ///
    /// Direction: Client -> Server.
    pub struct ClientSyncedInventory {
        const TAG = 138;
    }
}
