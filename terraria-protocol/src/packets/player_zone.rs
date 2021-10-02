use crate::packets::packet_struct;

packet_struct! {
    /// Player zone.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct PlayerZone {
        const TAG = 36;

        pub player_id: u8,
        /// 1 = Dungeon, 2 = Corruption, 4 =Holy, 8 = Meteor, 16 = Jungle, 32 = Snow, 64 = Crimson, 128 = Water Candle
        pub zoneflags1: u8,
        /// 1 = Peace Candle, 2 = Solar Tower, 4 = Vortex Tower, 8 = Nebula Tower, 16 = Stardust Tower, 32 = Desert, 64 = Glowshroom, 128 = Underground Desert
        pub zoneflags2: u8,
        /// 1 = Overworld, 2 = Dirt Layer, 4 = Rock Layer, 8 = Underworld, 16 = Beach, 32 = Rain, 64 = Sandstorm
        pub zoneflags3: u8,
        /// 1 = Old One's Army, 2 = Granite, 4 = Marble, 8 = Hive, 16 = Gem Cave, 32 = Lihzhard Temple, 64 = Graveyard
        pub zoneflags4: u8,
    }
}
