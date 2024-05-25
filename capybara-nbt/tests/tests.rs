use std::io::Read;

use capybara_nbt::{mcregion::Region, RootCompound};

#[test]
fn from_file() {
    let mut test_file = std::fs::File::open("./tests/level.dat").unwrap();

    let mut bytes = Vec::new();
    test_file.read_to_end(&mut bytes).unwrap();
    let mut bytes = bytes.as_slice();
    assert_eq!(
        format!("{:?}", RootCompound::parse(&mut bytes)),
        r#"Ok(RootCompound { tags: [Tag { name: "", tag: Compound([Tag { name: "Data", tag: Compound([Tag { name: "WanderingTraderSpawnChance", tag: Int(25) }, Tag { name: "BorderCenterZ", tag: Double(0.0) }, Tag { name: "Difficulty", tag: Byte(0) }, Tag { name: "BorderSizeLerpTime", tag: Long(0) }, Tag { name: "raining", tag: Byte(0) }, Tag { name: "Time", tag: Long(710) }, Tag { name: "GameType", tag: Int(1) }, Tag { name: "ServerBrands", tag: List([String("vanilla")]) }, Tag { name: "BorderCenterX", tag: Double(0.0) }, Tag { name: "BorderDamagePerBlock", tag: Double(0.2) }, Tag { name: "BorderWarningBlocks", tag: Double(5.0) }, Tag { name: "WorldGenSettings", tag: Compound([Tag { name: "bonus_chest", tag: Byte(0) }, Tag { name: "seed", tag: Long(6695648735511738860) }, Tag { name: "generate_features", tag: Byte(1) }, Tag { name: "dimensions", tag: Compound([Tag { name: "minecraft:overworld", tag: Compound([Tag { name: "generator", tag: Compound([Tag { name: "settings", tag: String("minecraft:overworld") }, Tag { name: "biome_source", tag: Compound([Tag { name: "preset", tag: String("minecraft:overworld") }, Tag { name: "type", tag: String("minecraft:multi_noise") }]) }, Tag { name: "type", tag: String("minecraft:noise") }]) }, Tag { name: "type", tag: String("minecraft:overworld") }]) }, Tag { name: "minecraft:the_nether", tag: Compound([Tag { name: "generator", tag: Compound([Tag { name: "settings", tag: String("minecraft:nether") }, Tag { name: "biome_source", tag: Compound([Tag { name: "preset", tag: String("minecraft:nether") }, Tag { name: "type", tag: String("minecraft:multi_noise") }]) }, Tag { name: "type", tag: String("minecraft:noise") }]) }, Tag { name: "type", tag: String("minecraft:the_nether") }]) }, Tag { name: "minecraft:the_end", tag: Compound([Tag { name: "generator", tag: Compound([Tag { name: "settings", tag: String("minecraft:end") }, Tag { name: "biome_source", tag: Compound([Tag { name: "type", tag: String("minecraft:the_end") }]) }, Tag { name: "type", tag: String("minecraft:noise") }]) }, Tag { name: "type", tag: String("minecraft:the_end") }]) }]) }]) }, Tag { name: "DragonFight", tag: Compound([Tag { name: "NeedsStateScanning", tag: Byte(1) }, Tag { name: "Gateways", tag: List([Int(4), Int(17), Int(10), Int(16), Int(15), Int(2), Int(3), Int(18), Int(9), Int(13), Int(8), Int(19), Int(12), Int(11), Int(5), Int(6), Int(7), Int(1), Int(14), Int(0)]) }, Tag { name: "DragonKilled", tag: Byte(1) }, Tag { name: "PreviouslyKilled", tag: Byte(1) }]) }, Tag { name: "BorderSizeLerpTarget", tag: Double(59999968.0) }, Tag { name: "Version", tag: Compound([Tag { name: "Snapshot", tag: Byte(0) }, Tag { name: "Series", tag: String("main") }, Tag { name: "Id", tag: Int(3337) }, Tag { name: "Name", tag: String("1.19.4") }]) }, Tag { name: "DayTime", tag: Long(710) }, Tag { name: "initialized", tag: Byte(1) }, Tag { name: "WasModded", tag: Byte(0) }, Tag { name: "allowCommands", tag: Byte(1) }, Tag { name: "WanderingTraderSpawnDelay", tag: Int(24000) }, Tag { name: "CustomBossEvents", tag: Compound([]) }, Tag { name: "GameRules", tag: Compound([Tag { name: "doWardenSpawning", tag: String("true") }, Tag { name: "globalSoundEvents", tag: String("true") }, Tag { name: "tntExplosionDropDecay", tag: String("false") }, Tag { name: "doFireTick", tag: String("true") }, Tag { name: "maxCommandChainLength", tag: String("65536") }, Tag { name: "doVinesSpread", tag: String("true") }, Tag { name: "fireDamage", tag: String("true") }, Tag { name: "reducedDebugInfo", tag: String("false") }, Tag { name: "waterSourceConversion", tag: String("true") }, Tag { name: "disableElytraMovementCheck", tag: String("false") }, Tag { name: "lavaSourceConversion", tag: String("false") }, Tag { name: "announceAdvancements", tag: String("true") }, Tag { name: "drowningDamage", tag: String("true") }, Tag { name: "commandBlockOutput", tag: String("true") }, Tag { name: "forgiveDeadPlayers", tag: String("true") }, Tag { name: "doMobSpawning", tag: String("true") }, Tag { name: "maxEntityCramming", tag: String("24") }, Tag { name: "disableRaids", tag: String("false") }, Tag { name: "doWeatherCycle", tag: String("true") }, Tag { name: "mobExplosionDropDecay", tag: String("true") }, Tag { name: "doDaylightCycle", tag: String("true") }, Tag { name: "showDeathMessages", tag: String("true") }, Tag { name: "doTileDrops", tag: String("true") }, Tag { name: "universalAnger", tag: String("false") }, Tag { name: "playersSleepingPercentage", tag: String("100") }, Tag { name: "snowAccumulationHeight", tag: String("1") }, Tag { name: "doInsomnia", tag: String("true") }, Tag { name: "blockExplosionDropDecay", tag: String("true") }, Tag { name: "doImmediateRespawn", tag: String("false") }, Tag { name: "naturalRegeneration", tag: String("true") }, Tag { name: "doMobLoot", tag: String("true") }, Tag { name: "fallDamage", tag: String("true") }, Tag { name: "keepInventory", tag: String("false") }, Tag { name: "doEntityDrops", tag: String("true") }, Tag { name: "doLimitedCrafting", tag: String("false") }, Tag { name: "mobGriefing", tag: String("true") }, Tag { name: "randomTickSpeed", tag: String("3") }, Tag { name: "spawnRadius", tag: String("10") }, Tag { name: "commandModificationBlockLimit", tag: String("32768") }, Tag { name: "doTraderSpawning", tag: String("true") }, Tag { name: "freezeDamage", tag: String("true") }, Tag { name: "logAdminCommands", tag: String("true") }, Tag { name: "spectatorsGenerateChunks", tag: String("true") }, Tag { name: "sendCommandFeedback", tag: String("true") }, Tag { name: "doPatrolSpawning", tag: String("true") }]) }, Tag { name: "Player", tag: Compound([Tag { name: "Brain", tag: Compound([Tag { name: "memories", tag: Compound([]) }]) }, Tag { name: "HurtByTimestamp", tag: Int(0) }, Tag { name: "SleepTimer", tag: Short(0) }, Tag { name: "Attributes", tag: List([Compound([Tag { name: "Base", tag: Double(0.10000000149011612) }, Tag { name: "Name", tag: String("minecraft:generic.movement_speed") }])]) }, Tag { name: "Invulnerable", tag: Byte(0) }, Tag { name: "FallFlying", tag: Byte(0) }, Tag { name: "PortalCooldown", tag: Int(0) }, Tag { name: "AbsorptionAmount", tag: Float(0.0) }, Tag { name: "abilities", tag: Compound([Tag { name: "invulnerable", tag: Byte(1) }, Tag { name: "mayfly", tag: Byte(1) }, Tag { name: "instabuild", tag: Byte(1) }, Tag { name: "walkSpeed", tag: Float(0.1) }, Tag { name: "mayBuild", tag: Byte(1) }, Tag { name: "flying", tag: Byte(0) }, Tag { name: "flySpeed", tag: Float(0.05) }]) }, Tag { name: "FallDistance", tag: Float(0.0) }, Tag { name: "recipeBook", tag: Compound([Tag { name: "recipes", tag: List([String("minecraft:crafting_table")]) }, Tag { name: "isBlastingFurnaceFilteringCraftable", tag: Byte(0) }, Tag { name: "isSmokerGuiOpen", tag: Byte(0) }, Tag { name: "isFilteringCraftable", tag: Byte(0) }, Tag { name: "toBeDisplayed", tag: List([String("minecraft:crafting_table")]) }, Tag { name: "isFurnaceGuiOpen", tag: Byte(0) }, Tag { name: "isGuiOpen", tag: Byte(0) }, Tag { name: "isFurnaceFilteringCraftable", tag: Byte(0) }, Tag { name: "isBlastingFurnaceGuiOpen", tag: Byte(0) }, Tag { name: "isSmokerFilteringCraftable", tag: Byte(0) }]) }, Tag { name: "DeathTime", tag: Short(0) }, Tag { name: "XpSeed", tag: Int(0) }, Tag { name: "XpTotal", tag: Int(0) }, Tag { name: "UUID", tag: IntArray([-79132136, 1795312482, -1668329806, 2116442193]) }, Tag { name: "playerGameType", tag: Int(1) }, Tag { name: "seenCredits", tag: Byte(0) }, Tag { name: "Motion", tag: List([Double(0.0), Double(-0.0784000015258789), Double(0.0)]) }, Tag { name: "Health", tag: Float(20.0) }, Tag { name: "foodSaturationLevel", tag: Float(5.8) }, Tag { name: "Air", tag: Short(300) }, Tag { name: "OnGround", tag: Byte(1) }, Tag { name: "Dimension", tag: String("minecraft:overworld") }, Tag { name: "Rotation", tag: List([Float(31.649858), Float(18.750109)]) }, Tag { name: "XpLevel", tag: Int(0) }, Tag { name: "warden_spawn_tracker", tag: Compound([Tag { name: "warning_level", tag: Int(0) }, Tag { name: "ticks_since_last_warning", tag: Int(708) }, Tag { name: "cooldown_ticks", tag: Int(0) }]) }, Tag { name: "Score", tag: Int(0) }, Tag { name: "Pos", tag: List([Double(-75.66743989671089), Double(64.0), Double(182.41095025123923)]) }, Tag { name: "Fire", tag: Short(-20) }, Tag { name: "XpP", tag: Float(0.0) }, Tag { name: "EnderItems", tag: List([]) }, Tag { name: "DataVersion", tag: Int(3337) }, Tag { name: "foodLevel", tag: Int(20) }, Tag { name: "foodExhaustionLevel", tag: Float(0.0) }, Tag { name: "HurtTime", tag: Short(0) }, Tag { name: "SelectedItemSlot", tag: Int(3) }, Tag { name: "Inventory", tag: List([Compound([Tag { name: "Slot", tag: Byte(0) }, Tag { name: "id", tag: String("minecraft:grass_block") }, Tag { name: "Count", tag: Byte(1) }]), Compound([Tag { name: "Slot", tag: Byte(1) }, Tag { name: "id", tag: String("minecraft:birch_stairs") }, Tag { name: "Count", tag: Byte(1) }]), Compound([Tag { name: "Slot", tag: Byte(2) }, Tag { name: "id", tag: String("minecraft:warped_button") }, Tag { name: "Count", tag: Byte(1) }]), Compound([Tag { name: "Slot", tag: Byte(3) }, Tag { name: "id", tag: String("minecraft:cookie") }, Tag { name: "Count", tag: Byte(1) }])]) }, Tag { name: "foodTickTimer", tag: Int(0) }]) }, Tag { name: "SpawnY", tag: Int(66) }, Tag { name: "rainTime", tag: Int(105675) }, Tag { name: "thunderTime", tag: Int(168835) }, Tag { name: "SpawnZ", tag: Int(192) }, Tag { name: "hardcore", tag: Byte(0) }, Tag { name: "DifficultyLocked", tag: Byte(0) }, Tag { name: "SpawnX", tag: Int(-80) }, Tag { name: "clearWeatherTime", tag: Int(0) }, Tag { name: "thundering", tag: Byte(0) }, Tag { name: "SpawnAngle", tag: Float(0.0) }, Tag { name: "version", tag: Int(19133) }, Tag { name: "BorderSafeZone", tag: Double(5.0) }, Tag { name: "LastPlayed", tag: Long(1698353224295) }, Tag { name: "BorderWarningTime", tag: Double(15.0) }, Tag { name: "ScheduledEvents", tag: List([]) }, Tag { name: "LevelName", tag: String("New World") }, Tag { name: "BorderSize", tag: Double(59999968.0) }, Tag { name: "DataVersion", tag: Int(3337) }, Tag { name: "DataPacks", tag: Compound([Tag { name: "Disabled", tag: List([String("bundle"), String("update_1_20")]) }, Tag { name: "Enabled", tag: List([String("vanilla")]) }]) }]) }]) }] })"#
    );
}

#[test]
fn region() {
    let mut test_file = std::fs::File::open("./tests/r.-2.-1.mca").unwrap();

    let mut bytes = Vec::new();
    test_file.read_to_end(&mut bytes).unwrap();
    let mut slice_data = bytes.as_slice();
    Region::parse(&mut slice_data).unwrap();
}

#[test]
fn region2() {
    let mut test_file = std::fs::File::open("./tests/region.mca").unwrap();

    let mut bytes = Vec::new();
    test_file.read_to_end(&mut bytes).unwrap();

    let mut slice_data = bytes.as_slice();
    Region::parse(&mut slice_data).unwrap();
}

#[test]
fn bigtest_wikivg() {
    let mut test_file = std::fs::File::open("./tests/bigtest.nbt").unwrap();

    let mut bytes = Vec::new();
    test_file.read_to_end(&mut bytes).unwrap();

    let mut slice_data = bytes.as_slice();
    RootCompound::parse(&mut slice_data).unwrap();
}

#[test]
fn complex_player() {
    let mut test_file = std::fs::File::open("./tests/complex_player.nbt").unwrap();

    let mut bytes = Vec::new();
    test_file.read_to_end(&mut bytes).unwrap();

    let mut slice_data = bytes.as_slice();
    RootCompound::parse(&mut slice_data).unwrap();
}
