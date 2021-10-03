use crate::serde::{PacketBody as _, SliceCursor};

macro_rules! define_packet {
    ($($variant:ident,)+) => {
        #[derive(PartialEq, Eq, Clone, Debug)]
        pub enum Packet {
            $($variant(crate::packets::$variant),)+
        }

        impl Packet {
            pub fn tag(&self) -> u8 {
                match self {
                    $(Self::$variant(_) => crate::packets::$variant::TAG,)+
                }
            }

            pub fn from_slice(slice: &mut [u8]) -> Self {
                let mut cursor = SliceCursor::new(slice);
                let tag = cursor.read::<u8>();
                let decoded = match tag {
                    $(
                        crate::packets::$variant::TAG =>
                            Self::$variant(crate::packets::$variant::from_body(&mut cursor)),
                    )+
                    tag => panic!("unknown tag {}", tag),
                };

                struct HexString<'a>(&'a [u8]);

                use std::fmt;
                impl<'a> fmt::Display for HexString<'a> {
                    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        for byte in self.0 {
                            write!(f, "{:02x}", byte)?;
                        }
                        Ok(())
                    }
                }

                let (slice, pos) = cursor.finish();
                assert_eq!(
                    pos,
                    slice.len(),
                    "incomplete read of packet: {}",
                    HexString(slice)
                );

                decoded
            }
        }

    };
}

define_packet!(
    /*   1 */ Connect,
    /*   2 */ Disconnect,
    /*   3 */ SetUserSlot,
    /*   4 */ PlayerInfo,
    /*   5 */ PlayerInventorySlot,
    /*   6 */ RequestWorldData,
    /*   7 */ WorldInfo,
    /*   8 */ RequestEssentialTiles,
    /*   9 */ Status,
    /*  10 */ SendSection,
    /*  11 */ SectionTileFrame,
    /*  12 */ SpawnPlayer,
    /*  13 */ UpdatePlayer,
    /*  14 */ PlayerActive,
    /*  15 */ Null,
    /*  16 */ PlayerHp,
    /*  17 */ ModifyTile,
    /*  18 */ Time,
    /*  19 */ DoorToggle,
    /*  20 */ SendTileSquare,
    /*  22 */ UpdateItemOwner,
    /*  23 */ NpcUpdate,
    /*  24 */ StrikeNpc,
    /*  27 */ ProjectileUpdate,
    /*  28 */ NpcStrike,
    /*  29 */ DestroyProjectile,
    /*  30 */ TogglePvp,
    /*  31 */ OpenChest,
    /*  32 */ UpdateChestItem,
    /*  33 */ SyncActiveChest,
    /*  34 */ PlaceChest,
    /*  35 */ HealEffect,
    /*  36 */ PlayerZone,
    /*  37 */ RequestPassword,
    /*  38 */ SendPassword,
    /*  39 */ RemoveItemOwner,
    /*  40 */ SetActiveNpc,
    /*  41 */ PlayerItemAnimation,
    /*  42 */ PlayerMana,
    /*  43 */ ManaEffect,
    /*  45 */ PlayerTeam,
    /*  46 */ RequestSign,
    /*  47 */ UpdateSign,
    /*  48 */ SetLiquid,
    /*  49 */ CompleteConnectionAndSpawn,
    /*  50 */ UpdatePlayerBuff,
    /*  51 */ SpecialNpcEffect,
    /*  52 */ Unlock,
    /*  53 */ AddNpcBuff,
    /*  54 */ UpdateNpcBuff,
    /*  55 */ AddPlayerBuff,
    /*  56 */ UpdateNpcName,
    /*  57 */ UpdateGoodEvil,
    /*  58 */ PlayMusicItem,
    /*  59 */ HitSwitch,
    /*  60 */ SetNpcHome,
    /*  61 */ SpawnBossInvasion,
    /*  62 */ PlayerDodge,
    /*  63 */ PaintTile,
    /*  64 */ PaintWall,
    /*  65 */ PlayerNpcTeleport,
    /*  66 */ HealOtherPlayer,
    /*  67 */ Placeholder,
    /*  68 */ ClientUuid,
    /*  69 */ GetChestName,
    /*  70 */ CatchNpc,
    /*  71 */ ReleaseNpc,
    /*  72 */ TravellingMerchantInventory,
    /*  73 */ TeleportationPotion,
    /*  74 */ AnglerQuest,
    /*  75 */ CompleteAnglerQuest,
    /*  76 */ AnglerQuests,
    /*  77 */ CreateTemporaryAnimation,
    /*  78 */ InvasionProgress,
    /*  79 */ PlaceObject,
    /*  80 */ SyncPlayerChestIndex,
    /*  81 */ CreateCombatText,
    /*  82 */ LoadNetModule,
    /*  83 */ SetNpcKillCount,
    /*  84 */ SetPlayerStealth,
    /*  85 */ QuickStash,
    /*  86 */ UpdateTileEntity,
    /*  87 */ PlaceTileEntity,
    /*  88 */ TweakItem,
    /*  89 */ PlaceItemFrame,
    /*  90 */ UpdateItemDrop2,
    /*  91 */ SyncEmoteBubble,
    /*  92 */ SyncExtraValue,
    /*  93 */ SocialHandshake,
    /*  95 */ KillPortal,
    /*  96 */ PlayerTeleportPortal,
    /*  97 */ PlayerNpcKilled,
    /*  98 */ SetEvent,
    /*  99 */ UpdateMinionTarget,
    /* 100 */ NpcTeleportPortal,
    /* 101 */ UpdateShieldStrengths,
    /* 102 */ NebulaLevelUp,
    /* 103 */ MoonLordCountdown,
    /* 104 */ NpcShopItem,
    /* 105 */ GemLockToggle,
    /* 106 */ PoofOfSmoke,
    /* 107 */ SmartTextMessage,
    /* 108 */ WiredCannonShot,
    /* 109 */ MassWire,
    /* 110 */ MassConsumeWire,
    /* 111 */ ToggleBirthdayParty,
    /* 112 */ GrowFx,
    /* 113 */ CrystalInvasionStart,
    /* 114 */ CrystalInvasionWipe,
    /* 115 */ SetMinionTarget,
    /* 116 */ CrystalInvasionWait,
    /* 117 */ PlayerHurt,
    /* 118 */ PlayerDeath,
    /* 119 */ CombatText,
    /* 120 */ Emoji,
    /* 121 */ DollSync,
    /* 122 */ InteractTileEntity,
    /* 123 */ PlaceWeaponRack,
    /* 124 */ HatRackSync,
    /* 125 */ SyncTilePicking,
    /* 126 */ SyncRevenge,
    /* 127 */ RemoveRevenge,
    /* 128 */ LandGolfBall,
    /* 129 */ ConnectionComplete,
    /* 130 */ FishOutNpc,
    /* 131 */ TamperWithNpc,
    /* 132 */ PlayLegacySound,
    /* 133 */ PlaceFood,
    /* 134 */ UpdatePlayerLuck,
    /* 135 */ DeadPlayer,
    /* 136 */ SyncMonsterType,
    /* 137 */ RequestNpcDebuff,
    /* 138 */ ClientSyncedInventory,
    /* 139 */ SetAsHost,
);
