use crate::packets::packet_struct;
use crate::structures::serializable_bitflags;

serializable_bitflags! {
    pub struct ZoneFlag: u32 {
        const DUNGEON = 0x0000_0001;
        const CORRUPTION = 0x0000_0002;
        const HOLY = 0x0000_0004;
        const METEOR = 0x0000_0008;
        const JUNGLE = 0x0000_0010;
        const SNOW = 0x0000_0020;
        const CRIMSON = 0x0000_0040;
        const WATER_CANDLE = 0x0000_0080;
        const PEACE_CANDLE = 0x0000_0100;
        const SOLAR_TOWER = 0x0000_0200;
        const VORTEX_TOWER = 0x0000_0400;
        const NEBULA_TOWER = 0x0000_0800;
        const STARDUST_TOWER = 0x0000_1000;
        const DESERT = 0x0000_2000;
        const GLOW_SHROOM = 0x0000_4000;
        const UNDERGROUND_DESERT = 0x0000_8000;
        const OVERWORLD = 0x0001_0000;
        const DIRT_LAYER = 0x0002_0000;
        const ROCK_LAYER = 0x0004_0000;
        const UNDERWORLD = 0x0008_0000;
        const BEACH = 0x0010_0000;
        const RAIN = 0x0020_0000;
        const SANDSTORM = 0x0040_0000;
        const OLD_ONE_ARMY = 0x0100_0000;
        const GRANITE = 0x0200_0000;
        const MARBLE = 0x0400_0000;
        const HIVE  = 0x0800_0000;
        const GEM_CAVE = 0x1000_0000;
        const LIZHARD_TEMPLE = 0x2000_0000;
        const GRAVEYARD = 0x4000_0000;
    }
}

packet_struct! {
    /// Player zone.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct PlayerZone {
        const TAG = 36;

        pub player_id: u8,
        pub flags: ZoneFlag,
    }
}
