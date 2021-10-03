use crate::serde::{
    serializable_struct, Deserializable, Error, PacketBody, Result, Serializable, SliceCursor,
};
use crate::structures::{LiquidType, NetString, Rgb, Vec2};

serializable_struct! {
    pub struct Liquid {
        pub y: i16,
        pub x: i16,
        pub amount: u8,
        pub ty: LiquidType,
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Bestiary {
    KillCount { npc_net_id: i16, kill_count: u16 },
    Sight { npc_net_id: i16 },
    Chat { npc_net_id: i16 },
}

impl Serializable for Bestiary {
    fn serialize(&self, cursor: &mut SliceCursor) -> Result<()> {
        match self {
            Self::KillCount {
                npc_net_id,
                kill_count,
            } => {
                cursor.write(npc_net_id)?;
                cursor.write(kill_count)?;
            }
            Self::Sight { npc_net_id } => {
                cursor.write(npc_net_id)?;
            }
            Self::Chat { npc_net_id } => {
                cursor.write(npc_net_id)?;
            }
        }
        Ok(())
    }
}

impl Deserializable for Bestiary {
    fn deserialize(cursor: &mut SliceCursor) -> Result<Self> {
        Ok(match cursor.read::<u8>()? {
            0 => Self::KillCount {
                npc_net_id: cursor.read()?,
                kill_count: cursor.read()?,
            },
            1 => Self::Sight {
                npc_net_id: cursor.read()?,
            },
            2 => Self::Chat {
                npc_net_id: cursor.read()?,
            },
            n => {
                return Err(Error::InvalidEnumValue {
                    enumeration: std::any::type_name::<Bestiary>(),
                    value: n as _,
                })
            }
        })
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum CreativePower {
    FreezeTime(i16),
    StartDayImmediately,
    StartNoonImmediately,
    StartNightImmediately,
    StartMidnightImmediately,
    GodmodePower(i16, [u8; 32]),
    ModifyWindDirectionAndStrength,
    ModifyRainPower,
    ModifyTimeRate(u8, i16, i16),
    FreezeRainPower(i16),
    FreezeWindDirectionAndStrength(i16),
    FarPlacementRangePower(i16, [u8; 32]),
    DifficultySliderPower(u8, i32),
    StopBiomeSpreadPower(i16),
    SpawnRateSliderPerPlayerPower([u8; 6]),
}

impl Serializable for CreativePower {
    fn serialize(&self, cursor: &mut SliceCursor) -> Result<()> {
        match self {
            Self::FreezeTime(val) => {
                cursor.write(&0)?;
                cursor.write(val)?;
            }
            Self::StartDayImmediately => cursor.write(&1)?,
            Self::StartNoonImmediately => cursor.write(&2)?,
            Self::StartNightImmediately => cursor.write(&3)?,
            Self::StartMidnightImmediately => cursor.write(&4)?,
            Self::GodmodePower(val, data) => {
                cursor.write(&5)?;
                cursor.write(val)?;
                for datum in data.iter() {
                    cursor.write(datum)?;
                }
            }
            Self::ModifyWindDirectionAndStrength => cursor.write(&6)?,
            Self::ModifyRainPower => cursor.write(&7)?,
            Self::ModifyTimeRate(ty, left, right) => {
                cursor.write(&8)?;
                cursor.write(ty)?;
                cursor.write(left)?;
                cursor.write(right)?;
            }
            Self::FreezeRainPower(val) => {
                cursor.write(&9)?;
                cursor.write(val)?;
            }
            Self::FreezeWindDirectionAndStrength(val) => {
                cursor.write(&10)?;
                cursor.write(val)?;
            }
            Self::FarPlacementRangePower(val, data) => {
                cursor.write(&11)?;
                cursor.write(val)?;
                for datum in data.iter() {
                    cursor.write(datum)?;
                }
            }
            Self::DifficultySliderPower(ty, val) => {
                cursor.write(&12)?;
                cursor.write(ty)?;
                cursor.write(val)?;
            }
            Self::StopBiomeSpreadPower(val) => {
                cursor.write(&13)?;
                cursor.write(val)?;
            }
            Self::SpawnRateSliderPerPlayerPower(data) => {
                cursor.write(&14)?;
                for datum in data.iter() {
                    cursor.write(datum)?;
                }
            }
        }
        Ok(())
    }
}

impl Deserializable for CreativePower {
    fn deserialize(cursor: &mut SliceCursor) -> Result<Self> {
        Ok(match cursor.read::<u8>()? {
            0 => CreativePower::FreezeTime(cursor.read()?),
            1 => CreativePower::StartDayImmediately,
            2 => CreativePower::StartNoonImmediately,
            3 => CreativePower::StartNightImmediately,
            4 => CreativePower::StartMidnightImmediately,
            5 => CreativePower::GodmodePower(cursor.read()?, {
                let mut data = [0; 32];
                for datum in data.iter_mut() {
                    *datum = cursor.read()?;
                }
                data
            }),
            6 => CreativePower::ModifyWindDirectionAndStrength,
            7 => CreativePower::ModifyRainPower,
            8 => CreativePower::ModifyTimeRate(cursor.read()?, cursor.read()?, cursor.read()?),
            9 => CreativePower::FreezeRainPower(cursor.read()?),
            10 => CreativePower::FreezeWindDirectionAndStrength(cursor.read()?),
            11 => CreativePower::FarPlacementRangePower(cursor.read()?, {
                let mut data = [0; 32];
                for datum in data.iter_mut() {
                    *datum = cursor.read()?;
                }
                data
            }),
            12 => CreativePower::DifficultySliderPower(cursor.read()?, cursor.read()?),
            13 => CreativePower::StopBiomeSpreadPower(cursor.read()?),
            14 => CreativePower::SpawnRateSliderPerPlayerPower({
                let mut data = [0; 6];
                for datum in data.iter_mut() {
                    *datum = cursor.read()?;
                }
                data
            }),
            n => {
                return Err(Error::InvalidEnumValue {
                    enumeration: std::any::type_name::<CreativePower>(),
                    value: n as _,
                })
            }
        })
    }
}

/// Load a network module.
///
/// Direction: Variable.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LoadNetModule {
    Liquid(Vec<Liquid>),
    // Client message is (command ID (text?), text)
    // Server message is (author ID, net text, Rgb)
    // TODO ClientText and ServerText are the same packet but unfortunately
    //      the only way to differentiate them is to know "who we are", that
    //      is, if we were compiled as the client we would send ClientText
    //      and receive ServerText, and vice versa.
    //
    //      Unfortunately, we currently pretend to always be the client (by
    //      always expecting to receive from server), which means it would
    //      not work if we were actually being used as a client.
    //
    //      We can probably determine if there's Rgb at the end of the message
    //      to determine who we are…
    ///
    /// The vanilla commands are (case-sensitive):
    /// * `"Emoji"`
    /// * `"Emote"`
    /// * `"Help"`
    /// * `"Playing"`
    /// * `"Party"`
    /// * `"RPS"`
    /// * `"Roll"`
    /// * `"Say"`
    ClientText {
        command: String,
        text: String,
    },
    ServerText {
        author: u8,
        text: NetString,
        color: Rgb,
    },
    Ping {
        pos: Vec2,
    },
    Ambience {
        player: u8,
        num: i32,
        ty: u8,
    },
    Bestiary(Bestiary),
    CreativeUnlocks {
        item_id: i16,
        sacrifice_count: u16,
    },
    CreativePowers(CreativePower),
    CreativeUnlocksPlayerReport {
        zero: u8,
        item_id: u16,
        amount: u16,
    },
    TeleportPylon {
        /// If 2, it's a use request. Otherwise, added or removed
        ty: u8,
        x: i16,
        y: i16,
        pylon_type: u8,
    },
    Particles {
        ty: u8,
        pos: Vec2,
        vel: Vec2,
        packed_shader_index: i32,
        /// Index of player who invoked this
        player: u8,
    },
    CreativePowerPermissions {
        zero: u8,
        power_id: u16,
        level: u8,
    },
}

impl Default for LoadNetModule {
    fn default() -> Self {
        Self::Liquid(Vec::new())
    }
}

impl PacketBody for LoadNetModule {
    const TAG: u8 = 82;

    fn write_body(&self, cursor: &mut SliceCursor) -> Result<()> {
        match self {
            LoadNetModule::Liquid(liquids) => {
                cursor.write(&(liquids.len() as u16))?;
                for liquid in liquids.iter() {
                    cursor.write(liquid)?;
                }
            }
            LoadNetModule::ClientText { command, text } => {
                cursor.write(&1u16)?;
                cursor.write(command)?;
                cursor.write(text)?;
            }
            LoadNetModule::ServerText {
                author,
                text,
                color,
            } => {
                cursor.write(&1u16)?;
                cursor.write(author)?;
                cursor.write(text)?;
                cursor.write(color)?;
            }
            LoadNetModule::Ping { pos } => {
                cursor.write(&2u16)?;
                cursor.write(pos)?;
            }
            LoadNetModule::Ambience { player, num, ty } => {
                cursor.write(&3u16)?;
                cursor.write(player)?;
                cursor.write(num)?;
                cursor.write(ty)?;
            }
            LoadNetModule::Bestiary(bestiary) => {
                cursor.write(&4u16)?;
                cursor.write(bestiary)?;
            }
            LoadNetModule::CreativeUnlocks {
                item_id,
                sacrifice_count,
            } => {
                cursor.write(&5u16)?;
                cursor.write(item_id)?;
                cursor.write(sacrifice_count)?;
            }
            LoadNetModule::CreativePowers(power) => {
                cursor.write(&6u16)?;
                cursor.write(power)?;
            }
            LoadNetModule::CreativeUnlocksPlayerReport {
                zero,
                item_id,
                amount,
            } => {
                cursor.write(&7u16)?;
                cursor.write(zero)?;
                cursor.write(item_id)?;
                cursor.write(amount)?;
            }
            LoadNetModule::TeleportPylon {
                ty,
                x,
                y,
                pylon_type,
            } => {
                cursor.write(&8u16)?;
                cursor.write(ty)?;
                cursor.write(x)?;
                cursor.write(y)?;
                cursor.write(pylon_type)?;
            }
            LoadNetModule::Particles {
                ty,
                pos,
                vel,
                packed_shader_index,
                player,
            } => {
                cursor.write(&9u16)?;
                cursor.write(ty)?;
                cursor.write(pos)?;
                cursor.write(vel)?;
                cursor.write(packed_shader_index)?;
                cursor.write(player)?;
            }
            LoadNetModule::CreativePowerPermissions {
                zero,
                power_id,
                level,
            } => {
                cursor.write(&10u16)?;
                cursor.write(zero)?;
                cursor.write(power_id)?;
                cursor.write(level)?;
            }
        }
        Ok(())
    }

    fn from_body(cursor: &mut SliceCursor) -> Result<Self> {
        Ok(match cursor.read::<u16>()? {
            0 => {
                let liquid_count = cursor.read::<u16>()?;
                LoadNetModule::Liquid(
                    (0..liquid_count)
                        .map(|_| cursor.read())
                        .collect::<Result<_>>()?,
                )
            }
            1 => LoadNetModule::ServerText {
                author: cursor.read()?,
                text: cursor.read()?,
                color: cursor.read()?,
            },
            2 => LoadNetModule::Ping {
                pos: cursor.read()?,
            },
            3 => LoadNetModule::Ambience {
                player: cursor.read()?,
                num: cursor.read()?,
                ty: cursor.read()?,
            },
            4 => LoadNetModule::Bestiary(cursor.read()?),
            5 => LoadNetModule::CreativeUnlocks {
                item_id: cursor.read()?,
                sacrifice_count: cursor.read()?,
            },
            6 => LoadNetModule::CreativePowers(cursor.read()?),
            7 => LoadNetModule::CreativeUnlocksPlayerReport {
                zero: cursor.read()?,
                item_id: cursor.read()?,
                amount: cursor.read()?,
            },
            8 => LoadNetModule::TeleportPylon {
                ty: cursor.read()?,
                x: cursor.read()?,
                y: cursor.read()?,
                pylon_type: cursor.read()?,
            },
            9 => LoadNetModule::Particles {
                ty: cursor.read()?,
                pos: cursor.read()?,
                vel: cursor.read()?,
                packed_shader_index: cursor.read()?,
                player: cursor.read()?,
            },
            10 => LoadNetModule::CreativePowerPermissions {
                zero: cursor.read()?,
                power_id: cursor.read()?,
                level: cursor.read()?,
            },
            n => {
                return Err(Error::InvalidEnumValue {
                    enumeration: std::any::type_name::<LoadNetModule>(),
                    value: n as _,
                })
            }
        })
    }
}
