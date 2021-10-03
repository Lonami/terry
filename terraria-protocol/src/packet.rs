use crate::packets::*;
use crate::serde::{PacketBody as _, SliceCursor};

use std::fmt;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Packet {
    Connect(Connect),                             // 1
    Disconnect(Disconnect),                       // 2
    SetUserSlot(SetUserSlot),                     // 3
    PlayerInfo(PlayerInfo),                       // 4
    PlayerInventorySlot(PlayerInventorySlot),     // 5
    RequestWorldData(RequestWorldData),           // 6
    WorldInfo(WorldInfo),                         // 7
    RequestEssentialTiles(RequestEssentialTiles), // 8
    Status(Status),                               // 9
    // TODO implement SendSection
    SendSection(SendSection),           // 10
    SectionTileFrame(SectionTileFrame), // 11
    SpawnPlayer(SpawnPlayer),           // 12
    UpdatePlayer(UpdatePlayer),         // 13
    PlayerActive(PlayerActive),         // 14
    Null(Null),                         // 15
    PlayerHp(PlayerHp),                 // 16
    ModifyTile(ModifyTile),             // 17
    Time(Time),                         // 18
    DoorToggle(DoorToggle),             // 19
    SendTileSquare(SendTileSquare),     // 20
    // 21 (replaced by 90)
    UpdateItemOwner(UpdateItemOwner), // 22
    // TODO all below need to be reviewed still:
    NpcUpdate(NpcUpdate), // 23
    StrikeNpc(StrikeNpc), // 24
    // 25 (null)
    // 26 (null)
    ProjectileUpdate(ProjectileUpdate),   // 27
    NpcStrike(NpcStrike),                 // 28
    DestroyProjectile(DestroyProjectile), // 29
    TogglePvp(TogglePvp),                 // 30
    OpenChest(OpenChest),                 // 31
    UpdateChestItem(UpdateChestItem),     // 32
    SyncActiveChest(SyncActiveChest),     // 33
    PlaceChest(PlaceChest),               // 34
    HealEffect(HealEffect),               // 35
    // TODO so many flags
    PlayerZone(PlayerZone),                   // 36
    RequestPassword(RequestPassword),         // 37
    SendPassword(SendPassword),               // 38
    RemoveItemOwner(RemoveItemOwner),         // 39
    SetActiveNpc(SetActiveNpc),               // 40
    PlayerItemAnimation(PlayerItemAnimation), // 41
    PlayerMana(PlayerMana),                   // 42
    ManaEffect(ManaEffect),                   // 43
    // 44 (null)
    PlayerTeam(PlayerTeam),                                 // 45
    RequestSign(RequestSign),                               // 46
    UpdateSign(UpdateSign),                                 // 47
    SetLiquid(SetLiquid),                                   // 48
    CompleteConnectionAndSpawn(CompleteConnectionAndSpawn), // 49
    UpdatePlayerBuff(UpdatePlayerBuff),                     // 50
    // TODO more enums
    SpecialNpcEffect(SpecialNpcEffect), // 51
    // TODO more enums
    Unlock(Unlock),                 // 52
    AddNpcBuff(AddNpcBuff),         // 53
    UpdateNpcBuff(UpdateNpcBuff),   // 54
    AddPlayerBuff(AddPlayerBuff),   // 55
    UpdateNpcName(UpdateNpcName),   // 56
    UpdateGoodEvil(UpdateGoodEvil), // 57
    PlayMusicItem(PlayMusicItem),   // 58
    HitSwitch(HitSwitch),           // 59
    SetNpcHome(SetNpcHome),         // 60
    // TODO more enums
    SpawnBossInvasion(SpawnBossInvasion), // 61
    // TODO more enums
    PlayerDodge(PlayerDodge), // 62
    PaintTile(PaintTile),     // 63
    PaintWall(PaintWall),     // 64
    // TODO more flags
    PlayerNpcTeleport(PlayerNpcTeleport), // 65
    HealOtherPlayer(HealOtherPlayer),     // 66
    Placeholder(Placeholder),             // 67
    ClientUuid(ClientUuid),               // 68
    GetChestName(GetChestName),           // 69
    CatchNpc(CatchNpc),                   // 70
    ReleaseNpc(ReleaseNpc),               // 71
    TravellingMerchantInventory(TravellingMerchantInventory), // 72
    // TODO enum
    TeleportationPotion(TeleportationPotion),           // 73
    AnglerQuest(AnglerQuest),                           // 74
    CompleteAnglerQuest(CompleteAnglerQuest),           // 75
    AnglerQuests(AnglerQuests),                         // 76
    CreateTemporaryAnimation(CreateTemporaryAnimation), // 77
    InvasionProgress(InvasionProgress),                 // 78
    PlaceObject(PlaceObject),                           // 79
    SyncPlayerChestIndex(SyncPlayerChestIndex),         // 80
    CreateCombatText(CreateCombatText),                 // 81
    LoadNetModule(LoadNetModule),                       // 82
    SetNpcKillCount(SetNpcKillCount),                   // 83
    SetPlayerStealth(SetPlayerStealth),                 // 84
    QuickStash(QuickStash),                             // 85
    UpdateTileEntity(UpdateTileEntity),                 // 86
    // TODO enum
    PlaceTileEntity(PlaceTileEntity), // 87
    // TODO flags
    TweakItem(TweakItem),             // 88
    PlaceItemFrame(PlaceItemFrame),   // 89
    UpdateItemDrop2(UpdateItemDrop2), // 90
    // TODO fancy enum
    SyncEmoteBubble(SyncEmoteBubble), // 91
    SyncExtraValue(SyncExtraValue),   // 92
    SocialHandshake(SocialHandshake), // 93
    // 94 (deprecated)
    KillPortal(KillPortal),                       // 95
    PlayerTeleportPortal(PlayerTeleportPortal),   // 96
    PlayerNpcKilled(PlayerNpcKilled),             // 97
    SetEvent(SetEvent),                           // 98
    UpdateMinionTarget(UpdateMinionTarget),       // 99
    NpcTeleportPortal(NpcTeleportPortal),         // 100
    UpdateShieldStrengths(UpdateShieldStrengths), // 101
    NebulaLevelUp(NebulaLevelUp),                 // 102
    MoonLordCountdown(MoonLordCountdown),         // 103
    NpcShopItem(NpcShopItem),                     // 104
    GemLockToggle(GemLockToggle),                 // 105
    PoofOfSmoke(PoofOfSmoke),                     // 106
    SmartTextMessage(SmartTextMessage),           // 107
    WiredCannonShot(WiredCannonShot),             // 108
    // TODO bitflags
    MassWire(MassWire),                       // 109
    MassConsumeWire(MassConsumeWire),         // 110
    ToggleBirthdayParty(ToggleBirthdayParty), // 111
    // TODO bitflags
    GrowFx(GrowFx),                             // 112
    CrystalInvasionStart(CrystalInvasionStart), // 113
    CrystalInvasionWipe(CrystalInvasionWipe),   // 114
    SetMinionTarget(SetMinionTarget),           // 115
    CrystalInvasionWait(CrystalInvasionWait),   // 116
    // TODO bitflags
    PlayerHurt(PlayerHurt),                 // 117
    PlayerDeath(PlayerDeath),               // 118
    CombatText(CombatText),                 // 119
    Emoji(Emoji),                           // 120
    DollSync(DollSync),                     // 121
    InteractTileEntity(InteractTileEntity), // 122
    PlaceWeaponRack(PlaceWeaponRack),       // 123
    HatRackSync(HatRackSync),               // 124
    SyncTilePicking(SyncTilePicking),       // 125
    SyncRevenge(SyncRevenge),               // 126
    RemoveRevenge(RemoveRevenge),           // 127
    LandGolfBall(LandGolfBall),             // 128
    ConnectionComplete(ConnectionComplete), // 129
    FishOutNpc(FishOutNpc),                 // 130
    TamperWithNpc(TamperWithNpc),           // 131
    // TODO enum
    PlayLegacySound(PlayLegacySound),             // 132
    PlaceFood(PlaceFood),                         // 133
    UpdatePlayerLuck(UpdatePlayerLuck),           // 134
    DeadPlayer(DeadPlayer),                       // 135
    SyncMonsterType(SyncMonsterType),             // 136
    RequestNpcDebuff(RequestNpcDebuff),           // 137
    ClientSyncedInventory(ClientSyncedInventory), // 138
    SetAsHost(SetAsHost),                         // 139
}

impl Packet {
    pub fn from_slice(slice: &mut [u8]) -> Self {
        let mut cursor = SliceCursor::new(slice);
        let tag = cursor.read::<u8>();
        // TODO too bad packet body is not serializable
        let decoded = match tag {
            Connect::TAG => Packet::Connect(Connect::from_body(&mut cursor)),
            Disconnect::TAG => Packet::Disconnect(Disconnect::from_body(&mut cursor)),
            SetUserSlot::TAG => Packet::SetUserSlot(SetUserSlot::from_body(&mut cursor)),
            PlayerInfo::TAG => Packet::PlayerInfo(PlayerInfo::from_body(&mut cursor)),
            PlayerInventorySlot::TAG => {
                Packet::PlayerInventorySlot(PlayerInventorySlot::from_body(&mut cursor))
            }
            RequestWorldData::TAG => {
                Packet::RequestWorldData(RequestWorldData::from_body(&mut cursor))
            }
            WorldInfo::TAG => Packet::WorldInfo(WorldInfo::from_body(&mut cursor)),
            RequestEssentialTiles::TAG => {
                Packet::RequestEssentialTiles(RequestEssentialTiles::from_body(&mut cursor))
            }
            Status::TAG => Packet::Status(Status::from_body(&mut cursor)),
            SendSection::TAG => Packet::SendSection(SendSection::from_body(&mut cursor)),
            SectionTileFrame::TAG => {
                Packet::SectionTileFrame(SectionTileFrame::from_body(&mut cursor))
            }
            SpawnPlayer::TAG => Packet::SpawnPlayer(SpawnPlayer::from_body(&mut cursor)),
            UpdatePlayer::TAG => Packet::UpdatePlayer(UpdatePlayer::from_body(&mut cursor)),
            PlayerActive::TAG => Packet::PlayerActive(PlayerActive::from_body(&mut cursor)),
            Null::TAG => Packet::Null(Null::from_body(&mut cursor)),
            PlayerHp::TAG => Packet::PlayerHp(PlayerHp::from_body(&mut cursor)),
            ModifyTile::TAG => Packet::ModifyTile(ModifyTile::from_body(&mut cursor)),
            Time::TAG => Packet::Time(Time::from_body(&mut cursor)),
            DoorToggle::TAG => Packet::DoorToggle(DoorToggle::from_body(&mut cursor)),
            SendTileSquare::TAG => Packet::SendTileSquare(SendTileSquare::from_body(&mut cursor)),
            21 => Packet::UpdateItemDrop2(UpdateItemDrop2::from_body(&mut cursor)),
            UpdateItemOwner::TAG => {
                Packet::UpdateItemOwner(UpdateItemOwner::from_body(&mut cursor))
            }
            NpcUpdate::TAG => Packet::NpcUpdate(NpcUpdate::from_body(&mut cursor)),
            StrikeNpc::TAG => Packet::StrikeNpc(StrikeNpc::from_body(&mut cursor)),
            25 => panic!("tag 25 is the null packet"),
            26 => panic!("tag 26 is the null packet"),
            ProjectileUpdate::TAG => {
                Packet::ProjectileUpdate(ProjectileUpdate::from_body(&mut cursor))
            }
            NpcStrike::TAG => Packet::NpcStrike(NpcStrike::from_body(&mut cursor)),
            DestroyProjectile::TAG => {
                Packet::DestroyProjectile(DestroyProjectile::from_body(&mut cursor))
            }
            TogglePvp::TAG => Packet::TogglePvp(TogglePvp::from_body(&mut cursor)),
            OpenChest::TAG => Packet::OpenChest(OpenChest::from_body(&mut cursor)),
            UpdateChestItem::TAG => {
                Packet::UpdateChestItem(UpdateChestItem::from_body(&mut cursor))
            }
            SyncActiveChest::TAG => {
                Packet::SyncActiveChest(SyncActiveChest::from_body(&mut cursor))
            }
            PlaceChest::TAG => Packet::PlaceChest(PlaceChest::from_body(&mut cursor)),
            HealEffect::TAG => Packet::HealEffect(HealEffect::from_body(&mut cursor)),
            PlayerZone::TAG => Packet::PlayerZone(PlayerZone::from_body(&mut cursor)),
            RequestPassword::TAG => {
                Packet::RequestPassword(RequestPassword::from_body(&mut cursor))
            }
            SendPassword::TAG => Packet::SendPassword(SendPassword::from_body(&mut cursor)),
            RemoveItemOwner::TAG => {
                Packet::RemoveItemOwner(RemoveItemOwner::from_body(&mut cursor))
            }
            SetActiveNpc::TAG => Packet::SetActiveNpc(SetActiveNpc::from_body(&mut cursor)),
            PlayerItemAnimation::TAG => {
                Packet::PlayerItemAnimation(PlayerItemAnimation::from_body(&mut cursor))
            }
            PlayerMana::TAG => Packet::PlayerMana(PlayerMana::from_body(&mut cursor)),
            ManaEffect::TAG => Packet::ManaEffect(ManaEffect::from_body(&mut cursor)),
            44 => panic!("tag 44 is the null packet"),
            PlayerTeam::TAG => Packet::PlayerTeam(PlayerTeam::from_body(&mut cursor)),
            RequestSign::TAG => Packet::RequestSign(RequestSign::from_body(&mut cursor)),
            UpdateSign::TAG => Packet::UpdateSign(UpdateSign::from_body(&mut cursor)),
            SetLiquid::TAG => Packet::SetLiquid(SetLiquid::from_body(&mut cursor)),
            CompleteConnectionAndSpawn::TAG => Packet::CompleteConnectionAndSpawn(
                CompleteConnectionAndSpawn::from_body(&mut cursor),
            ),
            UpdatePlayerBuff::TAG => {
                Packet::UpdatePlayerBuff(UpdatePlayerBuff::from_body(&mut cursor))
            }
            SpecialNpcEffect::TAG => {
                Packet::SpecialNpcEffect(SpecialNpcEffect::from_body(&mut cursor))
            }
            Unlock::TAG => Packet::Unlock(Unlock::from_body(&mut cursor)),
            AddNpcBuff::TAG => Packet::AddNpcBuff(AddNpcBuff::from_body(&mut cursor)),
            UpdateNpcBuff::TAG => Packet::UpdateNpcBuff(UpdateNpcBuff::from_body(&mut cursor)),
            AddPlayerBuff::TAG => Packet::AddPlayerBuff(AddPlayerBuff::from_body(&mut cursor)),
            UpdateNpcName::TAG => Packet::UpdateNpcName(UpdateNpcName::from_body(&mut cursor)),
            UpdateGoodEvil::TAG => Packet::UpdateGoodEvil(UpdateGoodEvil::from_body(&mut cursor)),
            PlayMusicItem::TAG => Packet::PlayMusicItem(PlayMusicItem::from_body(&mut cursor)),
            HitSwitch::TAG => Packet::HitSwitch(HitSwitch::from_body(&mut cursor)),
            SetNpcHome::TAG => Packet::SetNpcHome(SetNpcHome::from_body(&mut cursor)),
            SpawnBossInvasion::TAG => {
                Packet::SpawnBossInvasion(SpawnBossInvasion::from_body(&mut cursor))
            }
            PlayerDodge::TAG => Packet::PlayerDodge(PlayerDodge::from_body(&mut cursor)),
            PaintTile::TAG => Packet::PaintTile(PaintTile::from_body(&mut cursor)),
            PaintWall::TAG => Packet::PaintWall(PaintWall::from_body(&mut cursor)),
            PlayerNpcTeleport::TAG => {
                Packet::PlayerNpcTeleport(PlayerNpcTeleport::from_body(&mut cursor))
            }
            HealOtherPlayer::TAG => {
                Packet::HealOtherPlayer(HealOtherPlayer::from_body(&mut cursor))
            }
            Placeholder::TAG => Packet::Placeholder(Placeholder::from_body(&mut cursor)),
            ClientUuid::TAG => Packet::ClientUuid(ClientUuid::from_body(&mut cursor)),
            GetChestName::TAG => Packet::GetChestName(GetChestName::from_body(&mut cursor)),
            CatchNpc::TAG => Packet::CatchNpc(CatchNpc::from_body(&mut cursor)),
            ReleaseNpc::TAG => Packet::ReleaseNpc(ReleaseNpc::from_body(&mut cursor)),
            TravellingMerchantInventory::TAG => Packet::TravellingMerchantInventory(
                TravellingMerchantInventory::from_body(&mut cursor),
            ),
            TeleportationPotion::TAG => {
                Packet::TeleportationPotion(TeleportationPotion::from_body(&mut cursor))
            }
            AnglerQuest::TAG => Packet::AnglerQuest(AnglerQuest::from_body(&mut cursor)),
            CompleteAnglerQuest::TAG => {
                Packet::CompleteAnglerQuest(CompleteAnglerQuest::from_body(&mut cursor))
            }
            AnglerQuests::TAG => Packet::AnglerQuests(AnglerQuests::from_body(&mut cursor)),
            CreateTemporaryAnimation::TAG => {
                Packet::CreateTemporaryAnimation(CreateTemporaryAnimation::from_body(&mut cursor))
            }
            InvasionProgress::TAG => {
                Packet::InvasionProgress(InvasionProgress::from_body(&mut cursor))
            }
            PlaceObject::TAG => Packet::PlaceObject(PlaceObject::from_body(&mut cursor)),
            SyncPlayerChestIndex::TAG => {
                Packet::SyncPlayerChestIndex(SyncPlayerChestIndex::from_body(&mut cursor))
            }
            CreateCombatText::TAG => {
                Packet::CreateCombatText(CreateCombatText::from_body(&mut cursor))
            }
            LoadNetModule::TAG => Packet::LoadNetModule(LoadNetModule::from_body(&mut cursor)),
            SetNpcKillCount::TAG => {
                Packet::SetNpcKillCount(SetNpcKillCount::from_body(&mut cursor))
            }
            SetPlayerStealth::TAG => {
                Packet::SetPlayerStealth(SetPlayerStealth::from_body(&mut cursor))
            }
            QuickStash::TAG => Packet::QuickStash(QuickStash::from_body(&mut cursor)),
            UpdateTileEntity::TAG => {
                Packet::UpdateTileEntity(UpdateTileEntity::from_body(&mut cursor))
            }
            PlaceTileEntity::TAG => {
                Packet::PlaceTileEntity(PlaceTileEntity::from_body(&mut cursor))
            }
            TweakItem::TAG => Packet::TweakItem(TweakItem::from_body(&mut cursor)),
            PlaceItemFrame::TAG => Packet::PlaceItemFrame(PlaceItemFrame::from_body(&mut cursor)),
            UpdateItemDrop2::TAG => {
                Packet::UpdateItemDrop2(UpdateItemDrop2::from_body(&mut cursor))
            }
            SyncEmoteBubble::TAG => {
                Packet::SyncEmoteBubble(SyncEmoteBubble::from_body(&mut cursor))
            }
            SyncExtraValue::TAG => Packet::SyncExtraValue(SyncExtraValue::from_body(&mut cursor)),
            SocialHandshake::TAG => {
                Packet::SocialHandshake(SocialHandshake::from_body(&mut cursor))
            }
            94 => panic!("deprecated"),
            KillPortal::TAG => Packet::KillPortal(KillPortal::from_body(&mut cursor)),
            PlayerTeleportPortal::TAG => {
                Packet::PlayerTeleportPortal(PlayerTeleportPortal::from_body(&mut cursor))
            }
            PlayerNpcKilled::TAG => {
                Packet::PlayerNpcKilled(PlayerNpcKilled::from_body(&mut cursor))
            }
            SetEvent::TAG => Packet::SetEvent(SetEvent::from_body(&mut cursor)),
            UpdateMinionTarget::TAG => {
                Packet::UpdateMinionTarget(UpdateMinionTarget::from_body(&mut cursor))
            }
            NpcTeleportPortal::TAG => {
                Packet::NpcTeleportPortal(NpcTeleportPortal::from_body(&mut cursor))
            }
            UpdateShieldStrengths::TAG => {
                Packet::UpdateShieldStrengths(UpdateShieldStrengths::from_body(&mut cursor))
            }
            NebulaLevelUp::TAG => Packet::NebulaLevelUp(NebulaLevelUp::from_body(&mut cursor)),
            MoonLordCountdown::TAG => {
                Packet::MoonLordCountdown(MoonLordCountdown::from_body(&mut cursor))
            }
            NpcShopItem::TAG => Packet::NpcShopItem(NpcShopItem::from_body(&mut cursor)),
            GemLockToggle::TAG => Packet::GemLockToggle(GemLockToggle::from_body(&mut cursor)),
            PoofOfSmoke::TAG => Packet::PoofOfSmoke(PoofOfSmoke::from_body(&mut cursor)),
            SmartTextMessage::TAG => {
                Packet::SmartTextMessage(SmartTextMessage::from_body(&mut cursor))
            }
            WiredCannonShot::TAG => {
                Packet::WiredCannonShot(WiredCannonShot::from_body(&mut cursor))
            }
            MassWire::TAG => Packet::MassWire(MassWire::from_body(&mut cursor)),
            MassConsumeWire::TAG => {
                Packet::MassConsumeWire(MassConsumeWire::from_body(&mut cursor))
            }
            ToggleBirthdayParty::TAG => {
                Packet::ToggleBirthdayParty(ToggleBirthdayParty::from_body(&mut cursor))
            }
            GrowFx::TAG => Packet::GrowFx(GrowFx::from_body(&mut cursor)),
            CrystalInvasionStart::TAG => {
                Packet::CrystalInvasionStart(CrystalInvasionStart::from_body(&mut cursor))
            }
            CrystalInvasionWipe::TAG => {
                Packet::CrystalInvasionWipe(CrystalInvasionWipe::from_body(&mut cursor))
            }
            SetMinionTarget::TAG => {
                Packet::SetMinionTarget(SetMinionTarget::from_body(&mut cursor))
            }
            CrystalInvasionWait::TAG => {
                Packet::CrystalInvasionWait(CrystalInvasionWait::from_body(&mut cursor))
            }
            PlayerHurt::TAG => Packet::PlayerHurt(PlayerHurt::from_body(&mut cursor)),
            PlayerDeath::TAG => Packet::PlayerDeath(PlayerDeath::from_body(&mut cursor)),
            CombatText::TAG => Packet::CombatText(CombatText::from_body(&mut cursor)),
            Emoji::TAG => Packet::Emoji(Emoji::from_body(&mut cursor)),
            DollSync::TAG => Packet::DollSync(DollSync::from_body(&mut cursor)),
            InteractTileEntity::TAG => {
                Packet::InteractTileEntity(InteractTileEntity::from_body(&mut cursor))
            }
            PlaceWeaponRack::TAG => {
                Packet::PlaceWeaponRack(PlaceWeaponRack::from_body(&mut cursor))
            }
            HatRackSync::TAG => Packet::HatRackSync(HatRackSync::from_body(&mut cursor)),
            SyncTilePicking::TAG => {
                Packet::SyncTilePicking(SyncTilePicking::from_body(&mut cursor))
            }
            SyncRevenge::TAG => Packet::SyncRevenge(SyncRevenge::from_body(&mut cursor)),
            RemoveRevenge::TAG => Packet::RemoveRevenge(RemoveRevenge::from_body(&mut cursor)),
            LandGolfBall::TAG => Packet::LandGolfBall(LandGolfBall::from_body(&mut cursor)),
            ConnectionComplete::TAG => {
                Packet::ConnectionComplete(ConnectionComplete::from_body(&mut cursor))
            }
            FishOutNpc::TAG => Packet::FishOutNpc(FishOutNpc::from_body(&mut cursor)),
            TamperWithNpc::TAG => Packet::TamperWithNpc(TamperWithNpc::from_body(&mut cursor)),
            PlayLegacySound::TAG => {
                Packet::PlayLegacySound(PlayLegacySound::from_body(&mut cursor))
            }
            PlaceFood::TAG => Packet::PlaceFood(PlaceFood::from_body(&mut cursor)),
            UpdatePlayerLuck::TAG => {
                Packet::UpdatePlayerLuck(UpdatePlayerLuck::from_body(&mut cursor))
            }
            DeadPlayer::TAG => Packet::DeadPlayer(DeadPlayer::from_body(&mut cursor)),
            SyncMonsterType::TAG => {
                Packet::SyncMonsterType(SyncMonsterType::from_body(&mut cursor))
            }
            RequestNpcDebuff::TAG => {
                Packet::RequestNpcDebuff(RequestNpcDebuff::from_body(&mut cursor))
            }
            ClientSyncedInventory::TAG => {
                Packet::ClientSyncedInventory(ClientSyncedInventory::from_body(&mut cursor))
            }
            SetAsHost::TAG => Packet::SetAsHost(SetAsHost::from_body(&mut cursor)),

            tag => panic!("unknown tag {}", tag),
        };

        let (slice, pos) = cursor.finish();
        assert_eq!(
            pos,
            slice.len(),
            "incomplete read of packet: {}",
            HexString(slice)
        );

        decoded
    }

    pub fn tag(&self) -> u8 {
        match self {
            Packet::Connect(_) => Connect::TAG,
            Packet::Disconnect(_) => Disconnect::TAG,
            Packet::SetUserSlot(_) => SetUserSlot::TAG,
            Packet::PlayerInfo(_) => PlayerInfo::TAG,
            Packet::PlayerInventorySlot(_) => PlayerInventorySlot::TAG,
            Packet::RequestWorldData(_) => RequestWorldData::TAG,
            Packet::WorldInfo(_) => WorldInfo::TAG,
            Packet::RequestEssentialTiles(_) => RequestEssentialTiles::TAG,
            Packet::Status(_) => Status::TAG,
            Packet::SendSection(_) => SendSection::TAG,
            Packet::SectionTileFrame(_) => SectionTileFrame::TAG,
            Packet::SpawnPlayer(_) => SpawnPlayer::TAG,
            Packet::UpdatePlayer(_) => UpdatePlayer::TAG,
            Packet::PlayerActive(_) => PlayerActive::TAG,
            Packet::Null(_) => Null::TAG,
            Packet::PlayerHp(_) => PlayerHp::TAG,
            Packet::ModifyTile(_) => ModifyTile::TAG,
            Packet::Time(_) => Time::TAG,
            Packet::DoorToggle(_) => DoorToggle::TAG,
            Packet::SendTileSquare(_) => SendTileSquare::TAG,
            Packet::UpdateItemOwner(_) => UpdateItemOwner::TAG,
            Packet::NpcUpdate(_) => NpcUpdate::TAG,
            Packet::StrikeNpc(_) => StrikeNpc::TAG,
            Packet::ProjectileUpdate(_) => ProjectileUpdate::TAG,
            Packet::NpcStrike(_) => NpcStrike::TAG,
            Packet::DestroyProjectile(_) => DestroyProjectile::TAG,
            Packet::TogglePvp(_) => TogglePvp::TAG,
            Packet::OpenChest(_) => OpenChest::TAG,
            Packet::UpdateChestItem(_) => UpdateChestItem::TAG,
            Packet::SyncActiveChest(_) => SyncActiveChest::TAG,
            Packet::PlaceChest(_) => PlaceChest::TAG,
            Packet::HealEffect(_) => HealEffect::TAG,
            Packet::PlayerZone(_) => PlayerZone::TAG,
            Packet::RequestPassword(_) => RequestPassword::TAG,
            Packet::SendPassword(_) => SendPassword::TAG,
            Packet::RemoveItemOwner(_) => RemoveItemOwner::TAG,
            Packet::SetActiveNpc(_) => SetActiveNpc::TAG,
            Packet::PlayerItemAnimation(_) => PlayerItemAnimation::TAG,
            Packet::PlayerMana(_) => PlayerMana::TAG,
            Packet::ManaEffect(_) => ManaEffect::TAG,
            Packet::PlayerTeam(_) => PlayerTeam::TAG,
            Packet::RequestSign(_) => RequestSign::TAG,
            Packet::UpdateSign(_) => UpdateSign::TAG,
            Packet::SetLiquid(_) => SetLiquid::TAG,
            Packet::CompleteConnectionAndSpawn(_) => CompleteConnectionAndSpawn::TAG,
            Packet::UpdatePlayerBuff(_) => UpdatePlayerBuff::TAG,
            Packet::SpecialNpcEffect(_) => SpecialNpcEffect::TAG,
            Packet::Unlock(_) => Unlock::TAG,
            Packet::AddNpcBuff(_) => AddNpcBuff::TAG,
            Packet::UpdateNpcBuff(_) => UpdateNpcBuff::TAG,
            Packet::AddPlayerBuff(_) => AddPlayerBuff::TAG,
            Packet::UpdateNpcName(_) => UpdateNpcName::TAG,
            Packet::UpdateGoodEvil(_) => UpdateGoodEvil::TAG,
            Packet::PlayMusicItem(_) => PlayMusicItem::TAG,
            Packet::HitSwitch(_) => HitSwitch::TAG,
            Packet::SetNpcHome(_) => SetNpcHome::TAG,
            Packet::SpawnBossInvasion(_) => SpawnBossInvasion::TAG,
            Packet::PlayerDodge(_) => PlayerDodge::TAG,
            Packet::PaintTile(_) => PaintTile::TAG,
            Packet::PaintWall(_) => PaintWall::TAG,
            Packet::PlayerNpcTeleport(_) => PlayerNpcTeleport::TAG,
            Packet::HealOtherPlayer(_) => HealOtherPlayer::TAG,
            Packet::Placeholder(_) => Placeholder::TAG,
            Packet::ClientUuid(_) => ClientUuid::TAG,
            Packet::GetChestName(_) => GetChestName::TAG,
            Packet::CatchNpc(_) => CatchNpc::TAG,
            Packet::ReleaseNpc(_) => ReleaseNpc::TAG,
            Packet::TravellingMerchantInventory(_) => TravellingMerchantInventory::TAG,
            Packet::TeleportationPotion(_) => TeleportationPotion::TAG,
            Packet::AnglerQuest(_) => AnglerQuest::TAG,
            Packet::CompleteAnglerQuest(_) => CompleteAnglerQuest::TAG,
            Packet::AnglerQuests(_) => AnglerQuests::TAG,
            Packet::CreateTemporaryAnimation(_) => CreateTemporaryAnimation::TAG,
            Packet::InvasionProgress(_) => InvasionProgress::TAG,
            Packet::PlaceObject(_) => PlaceObject::TAG,
            Packet::SyncPlayerChestIndex(_) => SyncPlayerChestIndex::TAG,
            Packet::CreateCombatText(_) => CreateCombatText::TAG,
            Packet::LoadNetModule(_) => LoadNetModule::TAG,
            Packet::SetNpcKillCount(_) => SetNpcKillCount::TAG,
            Packet::SetPlayerStealth(_) => SetPlayerStealth::TAG,
            Packet::QuickStash(_) => QuickStash::TAG,
            Packet::UpdateTileEntity(_) => UpdateTileEntity::TAG,
            Packet::PlaceTileEntity(_) => PlaceTileEntity::TAG,
            Packet::TweakItem(_) => TweakItem::TAG,
            Packet::PlaceItemFrame(_) => PlaceItemFrame::TAG,
            Packet::UpdateItemDrop2(_) => UpdateItemDrop2::TAG,
            Packet::SyncEmoteBubble(_) => SyncEmoteBubble::TAG,
            Packet::SyncExtraValue(_) => SyncExtraValue::TAG,
            Packet::SocialHandshake(_) => SocialHandshake::TAG,
            Packet::KillPortal(_) => KillPortal::TAG,
            Packet::PlayerTeleportPortal(_) => PlayerTeleportPortal::TAG,
            Packet::PlayerNpcKilled(_) => PlayerNpcKilled::TAG,
            Packet::SetEvent(_) => SetEvent::TAG,
            Packet::UpdateMinionTarget(_) => UpdateMinionTarget::TAG,
            Packet::NpcTeleportPortal(_) => NpcTeleportPortal::TAG,
            Packet::UpdateShieldStrengths(_) => UpdateShieldStrengths::TAG,
            Packet::NebulaLevelUp(_) => NebulaLevelUp::TAG,
            Packet::MoonLordCountdown(_) => MoonLordCountdown::TAG,
            Packet::NpcShopItem(_) => NpcShopItem::TAG,
            Packet::GemLockToggle(_) => GemLockToggle::TAG,
            Packet::PoofOfSmoke(_) => PoofOfSmoke::TAG,
            Packet::SmartTextMessage(_) => SmartTextMessage::TAG,
            Packet::WiredCannonShot(_) => WiredCannonShot::TAG,
            Packet::MassWire(_) => MassWire::TAG,
            Packet::MassConsumeWire(_) => MassConsumeWire::TAG,
            Packet::ToggleBirthdayParty(_) => ToggleBirthdayParty::TAG,
            Packet::GrowFx(_) => GrowFx::TAG,
            Packet::CrystalInvasionStart(_) => CrystalInvasionStart::TAG,
            Packet::CrystalInvasionWipe(_) => CrystalInvasionWipe::TAG,
            Packet::SetMinionTarget(_) => SetMinionTarget::TAG,
            Packet::CrystalInvasionWait(_) => CrystalInvasionWait::TAG,
            Packet::PlayerHurt(_) => PlayerHurt::TAG,
            Packet::PlayerDeath(_) => PlayerDeath::TAG,
            Packet::CombatText(_) => CombatText::TAG,
            Packet::Emoji(_) => Emoji::TAG,
            Packet::DollSync(_) => DollSync::TAG,
            Packet::InteractTileEntity(_) => InteractTileEntity::TAG,
            Packet::PlaceWeaponRack(_) => PlaceWeaponRack::TAG,
            Packet::HatRackSync(_) => HatRackSync::TAG,
            Packet::SyncTilePicking(_) => SyncTilePicking::TAG,
            Packet::SyncRevenge(_) => SyncRevenge::TAG,
            Packet::RemoveRevenge(_) => RemoveRevenge::TAG,
            Packet::LandGolfBall(_) => LandGolfBall::TAG,
            Packet::ConnectionComplete(_) => ConnectionComplete::TAG,
            Packet::FishOutNpc(_) => FishOutNpc::TAG,
            Packet::TamperWithNpc(_) => TamperWithNpc::TAG,
            Packet::PlayLegacySound(_) => PlayLegacySound::TAG,
            Packet::PlaceFood(_) => PlaceFood::TAG,
            Packet::UpdatePlayerLuck(_) => UpdatePlayerLuck::TAG,
            Packet::DeadPlayer(_) => DeadPlayer::TAG,
            Packet::SyncMonsterType(_) => SyncMonsterType::TAG,
            Packet::RequestNpcDebuff(_) => RequestNpcDebuff::TAG,
            Packet::ClientSyncedInventory(_) => ClientSyncedInventory::TAG,
            Packet::SetAsHost(_) => SetAsHost::TAG,
        }
    }
}

struct HexString<'a>(&'a [u8]);

impl<'a> fmt::Display for HexString<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}
