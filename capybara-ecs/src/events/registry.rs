use crab_nbt::NbtCompound;

pub fn trim_pattern() {
    // let a = crab_nbt::nbt!(
    //     "root nbt_inner name",
    //     {
    //         "root":[crab_nbt::nbt!({}), ],
    //     } // [NbtCompound; {
    //        //             "name": "minecraft:coast",
    //        //             "id": 0,
    //        //             "element": {
    //        //                 "template_item": "minecraft:coast_armor_trim_smithing_template",
    //        //                 "description": {
    //        //                     "translate": "trim_pattern.minecraft.coast"
    //        //                 },
    //        //                 "asset_id": "minecraft:coast",
    //        //                 "decal": 0
    //        //             }
    //        //         },
    //        //         {
    //        //             "name": "minecraft:dune",
    //        //             "id": 1,
    //        //             "element": {
    //        //                 "template_item": "minecraft:dune_armor_trim_smithing_template",
    //        //                 "description": {
    //        //                     "translate": "trim_pattern.minecraft.dune"
    //        //                 },
    //        //                 "asset_id": "minecraft:dune",
    //        //                 "decal": 0
    //        //             }
    //        //         },
    //        //         {
    //        //             "name": "minecraft:eye",
    //        //             "id": 2,
    //        //             "element": {
    //        //                 "template_item": "minecraft:eye_armor_trim_smithing_template",
    //        //                 "description": {
    //        //                     "translate": "trim_pattern.minecraft.eye"
    //        //                 },
    //        //                 "asset_id": "minecraft:eye",
    //        //                 "decal": 0
    //        //             }
    //        //         },
    //        //         {
    //        //             "name": "minecraft:host",
    //        //             "id": 3,
    //        //             "element": {
    //        //                 "template_item": "minecraft:host_armor_trim_smithing_template",
    //        //                 "description": {
    //        //                     "translate": "trim_pattern.minecraft.host"
    //        //                 },
    //        //                 "asset_id": "minecraft:host",
    //        //                 "decal": 0
    //        //             }
    //        //         },
    //        //         {
    //        //             "name": "minecraft:raiser",
    //        //             "id": 4,
    //        //             "element": {
    //        //                 "template_item": "minecraft:raiser_armor_trim_smithing_template",
    //        //                 "description": {
    //        //                     "translate": "trim_pattern.minecraft.raiser"
    //        //                 },
    //        //                 "asset_id": "minecraft:raiser",
    //        //                 "decal": 0
    //        //             }
    //        //         },
    //        //         {
    //        //             "name": "minecraft:rib",
    //        //             "id": 5,
    //        //             "element": {
    //        //                 "template_item": "minecraft:rib_armor_trim_smithing_template",
    //        //                 "description": {
    //        //                     "translate": "trim_pattern.minecraft.rib"
    //        //                 },
    //        //                 "asset_id": "minecraft:rib",
    //        //                 "decal": 0
    //        //             }
    //        //         },
    //        //         {
    //        //             "name": "minecraft:sentry",
    //        //             "id": 6,
    //        //             "element": {
    //        //                 "template_item": "minecraft:sentry_armor_trim_smithing_template",
    //        //                 "description": {
    //        //                     "translate": "trim_pattern.minecraft.sentry"
    //        //                 },
    //        //                 "asset_id": "minecraft:sentry",
    //        //                 "decal": 0
    //        //             }
    //        //         },
    //        //         {
    //        //             "name": "minecraft:shaper",
    //        //             "id": 7,
    //        //             "element": {
    //        //                 "template_item": "minecraft:shaper_armor_trim_smithing_template",
    //        //                 "description": {
    //        //                     "translate": "trim_pattern.minecraft.shaper"
    //        //                 },
    //        //                 "asset_id": "minecraft:shaper",
    //        //                 "decal": 0
    //        //             }
    //        //         },
    //        //         {
    //        //             "name": "minecraft:silence",
    //        //             "id": 8,
    //        //             "element": {
    //        //                 "template_item": "minecraft:silence_armor_trim_smithing_template",
    //        //                 "description": {
    //        //                     "translate": "trim_pattern.minecraft.silence"
    //        //                 },
    //        //                 "asset_id": "minecraft:silence",
    //        //                 "decal": 0
    //        //             }
    //        //         },
    //        //         {
    //        //             "name": "minecraft:snout",
    //        //             "id": 9,
    //        //             "element": {
    //        //                 "template_item": "minecraft:snout_armor_trim_smithing_template",
    //        //                 "description": {
    //        //                     "translate": "trim_pattern.minecraft.snout"
    //        //                 },
    //        //                 "asset_id": "minecraft:snout",
    //        //                 "decal": 0
    //        //             }
    //        //         },
    //        //         {
    //        //             "name": "minecraft:spire",
    //        //             "id": 10,
    //        //             "element": {
    //        //                 "template_item": "minecraft:spire_armor_trim_smithing_template",
    //        //                 "description": {
    //        //                     "translate": "trim_pattern.minecraft.spire"
    //        //                 },
    //        //                 "asset_id": "minecraft:spire",
    //        //                 "decal": 0
    //        //             }
    //        //         },
    //        //         {
    //        //             "name": "minecraft:tide",
    //        //             "id": 11,
    //        //             "element": {
    //        //                 "template_item": "minecraft:tide_armor_trim_smithing_template",
    //        //                 "description": {
    //        //                     "translate": "trim_pattern.minecraft.tide"
    //        //                 },
    //        //                 "asset_id": "minecraft:tide",
    //        //                 "decal": 0
    //        //             }
    //        //         },
    //        //         {
    //        //             "name": "minecraft:vex",
    //        //             "id": 12,
    //        //             "element": {
    //        //                 "template_item": "minecraft:vex_armor_trim_smithing_template",
    //        //                 "description": {
    //        //                     "translate": "trim_pattern.minecraft.vex"
    //        //                 },
    //        //                 "asset_id": "minecraft:vex",
    //        //                 "decal": 0
    //        //             }
    //        //         },
    //        //         {
    //        //             "name": "minecraft:ward",
    //        //             "id": 13,
    //        //             "element": {
    //        //                 "template_item": "minecraft:ward_armor_trim_smithing_template",
    //        //                 "description": {
    //        //                     "translate": "trim_pattern.minecraft.ward"
    //        //                 },
    //        //                 "asset_id": "minecraft:ward",
    //        //                 "decal": 0
    //        //             }
    //        //         },
    //        //         {
    //        //             "name": "minecraft:wayfinder",
    //        //             "id": 14,
    //        //             "element": {
    //        //                 "template_item": "minecraft:wayfinder_armor_trim_smithing_template",
    //        //                 "description": {
    //        //                     "translate": "trim_pattern.minecraft.wayfinder"
    //        //                 },
    //        //                 "asset_id": "minecraft:wayfinder",
    //        //                 "decal": 0
    //        //             }
    //        //         },
    //        //         {
    //        //             "name": "minecraft:wild",
    //        //             "id": 15,
    //        //             "element": {
    //        //                 "template_item": "minecraft:wild_armor_trim_smithing_template",
    //        //                 "description": {
    //        //                     "translate": "trim_pattern.minecraft.wild"
    //        //                 },
    //        //                 "asset_id": "minecraft:wild",
    //        //                 "decal": 0
    //        //             }
    //        //         }]
    // );
}

// {
//     "minecraft:trim_pattern": {
//         "type": "minecraft:trim_pattern",
//         "value": [

//         ]
//     },
//     "minecraft:trim_material": {
//         "type": "minecraft:trim_material",
//         "value": [
//             {
//                 "name": "minecraft:amethyst",
//                 "id": 0,
//                 "element": {
//                     "ingredient": "minecraft:amethyst_shard",
//                     "asset_name": "amethyst",
//                     "item_model_index": 1.0,
//                     "description": {
//                         "color": "#9A5CC6",
//                         "translate": "trim_material.minecraft.amethyst"
//                     }
//                 }
//             },
//             {
//                 "name": "minecraft:copper",
//                 "id": 1,
//                 "element": {
//                     "ingredient": "minecraft:copper_ingot",
//                     "asset_name": "copper",
//                     "item_model_index": 0.5,
//                     "description": {
//                         "color": "#B4684",
//                         "translate": "trim_material.minecraft.copper"
//                     }
//                 }
//             },
//             {
//                 "name": "minecraft:diamond",
//                 "id": 2,
//                 "element": {
//                     "override_armor_materials": {
//                         "diamond": "diamond_darker"
//                     },
//                     "ingredient": "minecraft:diamond",
//                     "asset_name": "diamond",
//                     "item_model_index": 0.8,
//                     "description": {
//                         "color": "#6EECD2",
//                         "translate": "trim_material.minecraft.diamond"
//                     }
//                 }
//             },
//             {
//                 "name": "minecraft:emerald",
//                 "id": 3,
//                 "element": {
//                     "ingredient": "minecraft:emerald",
//                     "asset_name": "emerald",
//                     "item_model_index": 0.7,
//                     "description": {
//                         "color": "#11A036",
//                         "translate": "trim_material.minecraft.emerald"
//                     }
//                 }
//             },
//             {
//                 "name": "minecraft:gold",
//                 "id": 4,
//                 "element": {
//                     "override_armor_materials": {
//                         "gold": "gold_darker"
//                     },
//                     "ingredient": "minecraft:gold_ingot",
//                     "asset_name": "gold",
//                     "item_model_index": 0.6,
//                     "description": {
//                         "color": "#DEB12",
//                         "translate": "trim_material.minecraft.gold"
//                     }
//                 }
//             },
//             {
//                 "name": "minecraft:iron",
//                 "id": 5,
//                 "element": {
//                     "override_armor_materials": {
//                         "iron": "iron_darker"
//                     },
//                     "ingredient": "minecraft:iron_ingot",
//                     "asset_name": "iron",
//                     "item_model_index": 0.2,
//                     "description": {
//                         "color": "#ECECEC",
//                         "translate": "trim_material.minecraft.iron"
//                     }
//                 }
//             },
//             {
//                 "name": "minecraft:lapis",
//                 "id": 6,
//                 "element": {
//                     "ingredient": "minecraft:lapis_lazuli",
//                     "asset_name": "lapis",
//                     "item_model_index": 0.9,
//                     "description": {
//                         "color": "#416E97",
//                         "translate": "trim_material.minecraft.lapis"
//                     }
//                 }
//             },
//             {
//                 "name": "minecraft:netherite",
//                 "id": 7,
//                 "element": {
//                     "override_armor_materials": {
//                         "netherite": "netherite_darker"
//                     },
//                     "ingredient": "minecraft:netherite_ingot",
//                     "asset_name": "netherite",
//                     "item_model_index": 0.3,
//                     "description": {
//                         "color": "#625859",
//                         "translate": "trim_material.minecraft.netherite"
//                     }
//                 }
//             },
//             {
//                 "name": "minecraft:quartz",
//                 "id": 8,
//                 "element": {
//                     "ingredient": "minecraft:quartz",
//                     "asset_name": "quartz",
//                     "item_model_index": 0.1,
//                     "description": {
//                         "color": "#E34C4",
//                         "translate": "trim_material.minecraft.quartz"
//                     }
//                 }
//             },
//             {
//                 "name": "minecraft:redstone",
//                 "id": 9,
//                 "element": {
//                     "ingredient": "minecraft:redstone",
//                     "asset_name": "redstone",
//                     "item_model_index": 0.4,
//                     "description": {
//                         "color": "#971607",
//                         "translate": "trim_material.minecraft.redstone"
//                     }
//                 }
//             }
//         ]
//     },
//     "minecraft:chat_type": {
//         "type": "minecraft:chat_type",
//         "value": [
//             {
//                 "name": "minecraft:chat",
//                 "id": 0,
//                 "element": {
//                     "chat": {
//                         "translation_key": "chat.type.text",
//                         "parameters": [
//                             "sender",
//                             "content"
//                         ]
//                     },
//                     "narration": {
//                         "translation_key": "chat.type.text.narrate",
//                         "parameters": [
//                             "sender",
//                             "content"
//                         ]
//                     }
//                 }
//             },
//             {
//                 "name": "minecraft:emote_command",
//                 "id": 1,
//                 "element": {
//                     "chat": {
//                         "translation_key": "chat.type.emote",
//                         "parameters": [
//                             "sender",
//                             "content"
//                         ]
//                     },
//                     "narration": {
//                         "translation_key": "chat.type.emote",
//                         "parameters": [
//                             "sender",
//                             "content"
//                         ]
//                     }
//                 }
//             },
//             {
//                 "name": "minecraft:msg_command_incoming",
//                 "id": 2,
//                 "element": {
//                     "chat": {
//                         "translation_key": "commands.message.display.incoming",
//                         "style": {
//                             "color": "gray",
//                             "italic": 1
//                         },
//                         "parameters": [
//                             "sender",
//                             "content"
//                         ]
//                     },
//                     "narration": {
//                         "translation_key": "chat.type.text.narrate",
//                         "parameters": [
//                             "sender",
//                             "content"
//                         ]
//                     }
//                 }
//             },
//             {
//                 "name": "minecraft:msg_command_outgoing",
//                 "id": 3,
//                 "element": {
//                     "chat": {
//                         "translation_key": "commands.message.display.outgoing",
//                         "style": {
//                             "color": "gray",
//                             "italic": 1
//                         },
//                         "parameters": [
//                             "target",
//                             "content"
//                         ]
//                     },
//                     "narration": {
//                         "translation_key": "chat.type.text.narrate",
//                         "parameters": [
//                             "sender",
//                             "content"
//                         ]
//                     }
//                 }
//             },
//             {
//                 "name": "minecraft:say_command",
//                 "id": 4,
//                 "element": {
//                     "chat": {
//                         "translation_key": "chat.type.announcement",
//                         "parameters": [
//                             "sender",
//                             "content"
//                         ]
//                     },
//                     "narration": {
//                         "translation_key": "chat.type.text.narrate",
//                         "parameters": [
//                             "sender",
//                             "content"
//                         ]
//                     }
//                 }
//             },
//             {
//                 "name": "minecraft:team_msg_command_incoming",
//                 "id": 5,
//                 "element": {
//                     "chat": {
//                         "translation_key": "chat.type.team.text",
//                         "parameters": [
//                             "target",
//                             "sender",
//                             "content"
//                         ]
//                     },
//                     "narration": {
//                         "translation_key": "chat.type.text.narrate",
//                         "parameters": [
//                             "sender",
//                             "content"
//                         ]
//                     }
//                 }
//             },
//             {
//                 "name": "minecraft:team_msg_command_outgoing",
//                 "id": 6,
//                 "element": {
//                     "chat": {
//                         "translation_key": "chat.type.team.sent",
//                         "parameters": [
//                             "target",
//                             "sender",
//                             "content"
//                         ]
//                     },
//                     "narration": {
//                         "translation_key": "chat.type.text.narrate",
//                         "parameters": [
//                             "sender",
//                             "content"
//                         ]
//                     }
//                 }
//             }
//         ]
//     },
//     "minecraft:dimension_type": {
//         "type": "minecraft:dimension_type",
//         "value": [
//             {
//                 "name": "minecraft:overworld",
//                 "id": 0,
//                 "element": {
//                     "piglin_safe": 0,
//                     "natural": 1,
//                     "ambient_light": 0.0,
//                     "monster_spawn_block_light_limit": 0,
//                     "infiniburn": "#minecraft:infiniburn_overworld",
//                     "respawn_anchor_works": 0,
//                     "has_skylight": 1,
//                     "bed_works": 1,
//                     "effects": "minecraft:overworld",
//                     "has_raids": 1,
//                     "logical_height": 384,
//                     "coordinate_scale": 1.0,
//                     "monster_spawn_light_level": {
//                         "type": "minecraft:uniform",
//                         "value": {
//                             "min_inclusive": 0,
//                             "max_inclusive": 7
//                         }
//                     },
//                     "min_y": -64,
//                     "ultrawarm": 0,
//                     "has_ceiling": 0,
//                     "height": 384
//                 }
//             },
//             {
//                 "name": "minecraft:overworld_caves",
//                 "id": 1,
//                 "element": {
//                     "piglin_safe": 0,
//                     "natural": 1,
//                     "ambient_light": 0.0,
//                     "monster_spawn_block_light_limit": 0,
//                     "infiniburn": "#minecraft:infiniburn_overworld",
//                     "respawn_anchor_works": 0,
//                     "has_skylight": 1,
//                     "bed_works": 1,
//                     "effects": "minecraft:overworld",
//                     "has_raids": 1,
//                     "logical_height": 384,
//                     "coordinate_scale": 1.0,
//                     "monster_spawn_light_level": {
//                         "type": "minecraft:uniform",
//                         "value": {
//                             "min_inclusive": 0,
//                             "max_inclusive": 7
//                         }
//                     },
//                     "min_y": -64,
//                     "ultrawarm": 0,
//                     "has_ceiling": 1,
//                     "height": 384
//                 }
//             },
//             {
//                 "name": "minecraft:the_end",
//                 "id": 2,
//                 "element": {
//                     "piglin_safe": 0,
//                     "natural": 0,
//                     "ambient_light": 0.0,
//                     "monster_spawn_block_light_limit": 0,
//                     "infiniburn": "#minecraft:infiniburn_end",
//                     "respawn_anchor_works": 0,
//                     "has_skylight": 0,
//                     "bed_works": 0,
//                     "effects": "minecraft:the_end",
//                     "fixed_time": 6000,
//                     "has_raids": 1,
//                     "logical_height": 256,
//                     "coordinate_scale": 1.0,
//                     "monster_spawn_light_level": {
//                         "type": "minecraft:uniform",
//                         "value": {
//                             "min_inclusive": 0,
//                             "max_inclusive": 7
//                         }
//                     },
//                     "min_y": 0,
//                     "ultrawarm": 0,
//                     "has_ceiling": 0,
//                     "height": 256
//                 }
//             },
//             {
//                 "name": "minecraft:the_nether",
//                 "id": 3,
//                 "element": {
//                     "piglin_safe": 1,
//                     "natural": 0,
//                     "ambient_light": 0.1,
//                     "monster_spawn_block_light_limit": 15,
//                     "infiniburn": "#minecraft:infiniburn_nether",
//                     "respawn_anchor_works": 1,
//                     "has_skylight": 0,
//                     "bed_works": 0,
//                     "effects": "minecraft:the_nether",
//                     "fixed_time": 18000,
//                     "has_raids": 0,
//                     "logical_height": 128,
//                     "coordinate_scale": 8.0,
//                     "monster_spawn_light_level": 7,
//                     "min_y": 0,
//                     "ultrawarm": 1,
//                     "has_ceiling": 1,
//                     "height": 256
//                 }
//             }
//         ]
//     },
//     "minecraft:damage_type": {
//         "type": "minecraft:damage_type",
//         "value": [
//             {
//                 "name": "minecraft:arrow",
//                 "id": 0,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.1,
//                     "message_id": "arrow"
//                 }
//             },
//             {
//                 "name": "minecraft:bad_respawn_point",
//                 "id": 1,
//                 "element": {
//                     "scaling": "always",
//                     "exhaustion": 0.1,
//                     "message_id": "badRespawnPoint",
//                     "death_message_type": "intentional_game_design"
//                 }
//             },
//             {
//                 "name": "minecraft:cactus",
//                 "id": 2,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.1,
//                     "message_id": "cactus"
//                 }
//             },
//             {
//                 "name": "minecraft:cramming",
//                 "id": 3,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.0,
//                     "message_id": "cramming"
//                 }
//             },
//             {
//                 "name": "minecraft:dragon_breath",
//                 "id": 4,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.0,
//                     "message_id": "dragonBreath"
//                 }
//             },
//             {
//                 "name": "minecraft:drown",
//                 "id": 5,
//                 "element": {
//                     "effects": "drowning",
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.0,
//                     "message_id": "drown"
//                 }
//             },
//             {
//                 "name": "minecraft:dry_out",
//                 "id": 6,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.1,
//                     "message_id": "dryout"
//                 }
//             },
//             {
//                 "name": "minecraft:explosion",
//                 "id": 7,
//                 "element": {
//                     "scaling": "always",
//                     "exhaustion": 0.1,
//                     "message_id": "explosion"
//                 }
//             },
//             {
//                 "name": "minecraft:fall",
//                 "id": 8,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.0,
//                     "message_id": "fall",
//                     "death_message_type": "fall_variants"
//                 }
//             },
//             {
//                 "name": "minecraft:falling_anvil",
//                 "id": 9,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.1,
//                     "message_id": "anvil"
//                 }
//             },
//             {
//                 "name": "minecraft:falling_block",
//                 "id": 10,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.1,
//                     "message_id": "fallingBlock"
//                 }
//             },
//             {
//                 "name": "minecraft:falling_stalactite",
//                 "id": 11,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.1,
//                     "message_id": "fallingStalactite"
//                 }
//             },
//             {
//                 "name": "minecraft:fireball",
//                 "id": 12,
//                 "element": {
//                     "effects": "burning",
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.1,
//                     "message_id": "fireball"
//                 }
//             },
//             {
//                 "name": "minecraft:fireworks",
//                 "id": 13,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.1,
//                     "message_id": "fireworks"
//                 }
//             },
//             {
//                 "name": "minecraft:fly_into_wall",
//                 "id": 14,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.0,
//                     "message_id": "flyIntoWall"
//                 }
//             },
//             {
//                 "name": "minecraft:freeze",
//                 "id": 15,
//                 "element": {
//                     "effects": "freezing",
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.0,
//                     "message_id": "freeze"
//                 }
//             },
//             {
//                 "name": "minecraft:generic",
//                 "id": 16,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.0,
//                     "message_id": "generic"
//                 }
//             },
//             {
//                 "name": "minecraft:generic_kill",
//                 "id": 17,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.0,
//                     "message_id": "genericKill"
//                 }
//             },
//             {
//                 "name": "minecraft:hot_floor",
//                 "id": 18,
//                 "element": {
//                     "effects": "burning",
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.1,
//                     "message_id": "hotFloor"
//                 }
//             },
//             {
//                 "name": "minecraft:in_fire",
//                 "id": 19,
//                 "element": {
//                     "effects": "burning",
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.1,
//                     "message_id": "inFire"
//                 }
//             },
//             {
//                 "name": "minecraft:in_wall",
//                 "id": 20,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.0,
//                     "message_id": "inWall"
//                 }
//             },
//             {
//                 "name": "minecraft:indirect_magic",
//                 "id": 21,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.0,
//                     "message_id": "indirectMagic"
//                 }
//             },
//             {
//                 "name": "minecraft:lava",
//                 "id": 22,
//                 "element": {
//                     "effects": "burning",
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.1,
//                     "message_id": "lava"
//                 }
//             },
//             {
//                 "name": "minecraft:lightning_bolt",
//                 "id": 23,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.1,
//                     "message_id": "lightningBolt"
//                 }
//             },
//             {
//                 "name": "minecraft:magic",
//                 "id": 24,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.0,
//                     "message_id": "magic"
//                 }
//             },
//             {
//                 "name": "minecraft:mob_attack",
//                 "id": 25,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.1,
//                     "message_id": "mob"
//                 }
//             },
//             {
//                 "name": "minecraft:mob_attack_no_aggro",
//                 "id": 26,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.1,
//                     "message_id": "mob"
//                 }
//             },
//             {
//                 "name": "minecraft:mob_projectile",
//                 "id": 27,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.1,
//                     "message_id": "mob"
//                 }
//             },
//             {
//                 "name": "minecraft:on_fire",
//                 "id": 28,
//                 "element": {
//                     "effects": "burning",
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.0,
//                     "message_id": "onFire"
//                 }
//             },
//             {
//                 "name": "minecraft:out_of_world",
//                 "id": 29,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.0,
//                     "message_id": "outOfWorld"
//                 }
//             },
//             {
//                 "name": "minecraft:outside_border",
//                 "id": 30,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.0,
//                     "message_id": "outsideBorder"
//                 }
//             },
//             {
//                 "name": "minecraft:player_attack",
//                 "id": 31,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.1,
//                     "message_id": "player"
//                 }
//             },
//             {
//                 "name": "minecraft:player_explosion",
//                 "id": 32,
//                 "element": {
//                     "scaling": "always",
//                     "exhaustion": 0.1,
//                     "message_id": "explosion.player"
//                 }
//             },
//             {
//                 "name": "minecraft:sonic_boom",
//                 "id": 33,
//                 "element": {
//                     "scaling": "always",
//                     "exhaustion": 0.0,
//                     "message_id": "sonic_boom"
//                 }
//             },
//             {
//                 "name": "minecraft:stalagmite",
//                 "id": 34,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.0,
//                     "message_id": "stalagmite"
//                 }
//             },
//             {
//                 "name": "minecraft:starve",
//                 "id": 35,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.0,
//                     "message_id": "starve"
//                 }
//             },
//             {
//                 "name": "minecraft:sting",
//                 "id": 36,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.1,
//                     "message_id": "sting"
//                 }
//             },
//             {
//                 "name": "minecraft:sweet_berry_bush",
//                 "id": 37,
//                 "element": {
//                     "effects": "poking",
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.1,
//                     "message_id": "sweetBerryBush"
//                 }
//             },
//             {
//                 "name": "minecraft:thorns",
//                 "id": 38,
//                 "element": {
//                     "effects": "thorns",
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.1,
//                     "message_id": "thorns"
//                 }
//             },
//             {
//                 "name": "minecraft:thrown",
//                 "id": 39,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.1,
//                     "message_id": "thrown"
//                 }
//             },
//             {
//                 "name": "minecraft:trident",
//                 "id": 40,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.1,
//                     "message_id": "trident"
//                 }
//             },
//             {
//                 "name": "minecraft:unattributed_fireball",
//                 "id": 41,
//                 "element": {
//                     "effects": "burning",
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.1,
//                     "message_id": "onFire"
//                 }
//             },
//             {
//                 "name": "minecraft:wither",
//                 "id": 42,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.0,
//                     "message_id": "wither"
//                 }
//             },
//             {
//                 "name": "minecraft:wither_skull",
//                 "id": 43,
//                 "element": {
//                     "scaling": "when_caused_by_living_non_player",
//                     "exhaustion": 0.1,
//                     "message_id": "witherSkull"
//                 }
//             }
//         ]
//     },
//     "minecraft:worldgen/biome": {
//         "type": "minecraft:worldgen/biome",
//         "value": [
//             {
//                 "name": "minecraft:badlands",
//                 "id": 0,
//                 "element": {
//                     "effects": {
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.overworld.badlands",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 7254527,
//                         "grass_color": 9470285,
//                         "foliage_color": 10387789,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 0,
//                     "temperature": 2.0,
//                     "downfall": 0.0
//                 }
//             },
//             {
//                 "name": "minecraft:bamboo_jungle",
//                 "id": 1,
//                 "element": {
//                     "effects": {
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.overworld.bamboo_jungle",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 7842047,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.95,
//                     "downfall": 0.9
//                 }
//             },
//             {
//                 "name": "minecraft:basalt_deltas",
//                 "id": 2,
//                 "element": {
//                     "effects": {
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.nether.basalt_deltas",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 7254527,
//                         "ambient_sound": "minecraft:ambient.basalt_deltas.loop",
//                         "additions_sound": {
//                             "sound": "minecraft:ambient.basalt_deltas.additions",
//                             "tick_chance": 0.0111
//                         },
//                         "particle": {
//                             "probability": 0.118093334,
//                             "options": {
//                                 "type": "minecraft:white_ash"
//                             }
//                         },
//                         "water_fog_color": 329011,
//                         "fog_color": 6840176,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.basalt_deltas.mood",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 0,
//                     "temperature": 2.0,
//                     "downfall": 0.0
//                 }
//             },
//             {
//                 "name": "minecraft:beach",
//                 "id": 3,
//                 "element": {
//                     "effects": {
//                         "sky_color": 7907327,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.8,
//                     "downfall": 0.4
//                 }
//             },
//             {
//                 "name": "minecraft:birch_forest",
//                 "id": 4,
//                 "element": {
//                     "effects": {
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.overworld.forest",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 8037887,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.6,
//                     "downfall": 0.6
//                 }
//             },
//             {
//                 "name": "minecraft:cherry_grove",
//                 "id": 5,
//                 "element": {
//                     "effects": {
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.overworld.cherry_grove",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 8103167,
//                         "grass_color": 11983713,
//                         "foliage_color": 11983713,
//                         "water_fog_color": 6141935,
//                         "fog_color": 12638463,
//                         "water_color": 6141935,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.5,
//                     "downfall": 0.8
//                 }
//             },
//             {
//                 "name": "minecraft:cold_ocean",
//                 "id": 6,
//                 "element": {
//                     "effects": {
//                         "sky_color": 8103167,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4020182,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.5,
//                     "downfall": 0.5
//                 }
//             },
//             {
//                 "name": "minecraft:crimson_forest",
//                 "id": 7,
//                 "element": {
//                     "effects": {
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.nether.crimson_forest",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 7254527,
//                         "ambient_sound": "minecraft:ambient.crimson_forest.loop",
//                         "additions_sound": {
//                             "sound": "minecraft:ambient.crimson_forest.additions",
//                             "tick_chance": 0.0111
//                         },
//                         "particle": {
//                             "probability": 0.025,
//                             "options": {
//                                 "type": "minecraft:crimson_spore"
//                             }
//                         },
//                         "water_fog_color": 329011,
//                         "fog_color": 3343107,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.crimson_forest.mood",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 0,
//                     "temperature": 2.0,
//                     "downfall": 0.0
//                 }
//             },
//             {
//                 "name": "minecraft:dark_forest",
//                 "id": 8,
//                 "element": {
//                     "effects": {
//                         "grass_color_modifier": "dark_forest",
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.overworld.forest",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 7972607,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.7,
//                     "downfall": 0.8
//                 }
//             },
//             {
//                 "name": "minecraft:deep_cold_ocean",
//                 "id": 9,
//                 "element": {
//                     "effects": {
//                         "sky_color": 8103167,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4020182,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.5,
//                     "downfall": 0.5
//                 }
//             },
//             {
//                 "name": "minecraft:deep_dark",
//                 "id": 10,
//                 "element": {
//                     "effects": {
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.overworld.deep_dark",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 7907327,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.8,
//                     "downfall": 0.4
//                 }
//             },
//             {
//                 "name": "minecraft:deep_frozen_ocean",
//                 "id": 11,
//                 "element": {
//                     "effects": {
//                         "sky_color": 8103167,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 3750089,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.5,
//                     "downfall": 0.5,
//                     "temperature_modifier": "frozen"
//                 }
//             },
//             {
//                 "name": "minecraft:deep_lukewarm_ocean",
//                 "id": 12,
//                 "element": {
//                     "effects": {
//                         "sky_color": 8103167,
//                         "water_fog_color": 267827,
//                         "fog_color": 12638463,
//                         "water_color": 4566514,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.5,
//                     "downfall": 0.5
//                 }
//             },
//             {
//                 "name": "minecraft:deep_ocean",
//                 "id": 13,
//                 "element": {
//                     "effects": {
//                         "sky_color": 8103167,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.5,
//                     "downfall": 0.5
//                 }
//             },
//             {
//                 "name": "minecraft:desert",
//                 "id": 14,
//                 "element": {
//                     "effects": {
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.overworld.desert",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 7254527,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 0,
//                     "temperature": 2.0,
//                     "downfall": 0.0
//                 }
//             },
//             {
//                 "name": "minecraft:dripstone_caves",
//                 "id": 15,
//                 "element": {
//                     "effects": {
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.overworld.dripstone_caves",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 7907327,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.8,
//                     "downfall": 0.4
//                 }
//             },
//             {
//                 "name": "minecraft:end_barrens",
//                 "id": 16,
//                 "element": {
//                     "effects": {
//                         "sky_color": 0,
//                         "water_fog_color": 329011,
//                         "fog_color": 10518688,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 0,
//                     "temperature": 0.5,
//                     "downfall": 0.5
//                 }
//             },
//             {
//                 "name": "minecraft:end_highlands",
//                 "id": 17,
//                 "element": {
//                     "effects": {
//                         "sky_color": 0,
//                         "water_fog_color": 329011,
//                         "fog_color": 10518688,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 0,
//                     "temperature": 0.5,
//                     "downfall": 0.5
//                 }
//             },
//             {
//                 "name": "minecraft:end_midlands",
//                 "id": 18,
//                 "element": {
//                     "effects": {
//                         "sky_color": 0,
//                         "water_fog_color": 329011,
//                         "fog_color": 10518688,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 0,
//                     "temperature": 0.5,
//                     "downfall": 0.5
//                 }
//             },
//             {
//                 "name": "minecraft:eroded_badlands",
//                 "id": 19,
//                 "element": {
//                     "effects": {
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.overworld.badlands",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 7254527,
//                         "grass_color": 9470285,
//                         "foliage_color": 10387789,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 0,
//                     "temperature": 2.0,
//                     "downfall": 0.0
//                 }
//             },
//             {
//                 "name": "minecraft:flower_forest",
//                 "id": 20,
//                 "element": {
//                     "effects": {
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.overworld.flower_forest",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 7972607,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.7,
//                     "downfall": 0.8
//                 }
//             },
//             {
//                 "name": "minecraft:forest",
//                 "id": 21,
//                 "element": {
//                     "effects": {
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.overworld.forest",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 7972607,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.7,
//                     "downfall": 0.8
//                 }
//             },
//             {
//                 "name": "minecraft:frozen_ocean",
//                 "id": 22,
//                 "element": {
//                     "effects": {
//                         "sky_color": 8364543,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 3750089,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.0,
//                     "downfall": 0.5,
//                     "temperature_modifier": "frozen"
//                 }
//             },
//             {
//                 "name": "minecraft:frozen_peaks",
//                 "id": 23,
//                 "element": {
//                     "effects": {
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.overworld.frozen_peaks",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 8756735,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": -0.7,
//                     "downfall": 0.9
//                 }
//             },
//             {
//                 "name": "minecraft:frozen_river",
//                 "id": 24,
//                 "element": {
//                     "effects": {
//                         "sky_color": 8364543,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 3750089,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.0,
//                     "downfall": 0.5
//                 }
//             },
//             {
//                 "name": "minecraft:grove",
//                 "id": 25,
//                 "element": {
//                     "effects": {
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.overworld.grove",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 8495359,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": -0.2,
//                     "downfall": 0.8
//                 }
//             },
//             {
//                 "name": "minecraft:ice_spikes",
//                 "id": 26,
//                 "element": {
//                     "effects": {
//                         "sky_color": 8364543,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.0,
//                     "downfall": 0.5
//                 }
//             },
//             {
//                 "name": "minecraft:jagged_peaks",
//                 "id": 27,
//                 "element": {
//                     "effects": {
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.overworld.jagged_peaks",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 8756735,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": -0.7,
//                     "downfall": 0.9
//                 }
//             },
//             {
//                 "name": "minecraft:jungle",
//                 "id": 28,
//                 "element": {
//                     "effects": {
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.overworld.jungle",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 7842047,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.95,
//                     "downfall": 0.9
//                 }
//             },
//             {
//                 "name": "minecraft:lukewarm_ocean",
//                 "id": 29,
//                 "element": {
//                     "effects": {
//                         "sky_color": 8103167,
//                         "water_fog_color": 267827,
//                         "fog_color": 12638463,
//                         "water_color": 4566514,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.5,
//                     "downfall": 0.5
//                 }
//             },
//             {
//                 "name": "minecraft:lush_caves",
//                 "id": 30,
//                 "element": {
//                     "effects": {
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.overworld.lush_caves",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 8103167,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.5,
//                     "downfall": 0.5
//                 }
//             },
//             {
//                 "name": "minecraft:mangrove_swamp",
//                 "id": 31,
//                 "element": {
//                     "effects": {
//                         "grass_color_modifier": "swamp",
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.overworld.swamp",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 7907327,
//                         "foliage_color": 9285927,
//                         "water_fog_color": 5077600,
//                         "fog_color": 12638463,
//                         "water_color": 3832426,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.8,
//                     "downfall": 0.9
//                 }
//             },
//             {
//                 "name": "minecraft:meadow",
//                 "id": 32,
//                 "element": {
//                     "effects": {
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.overworld.meadow",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 8103167,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 937679,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.5,
//                     "downfall": 0.8
//                 }
//             },
//             {
//                 "name": "minecraft:mushroom_fields",
//                 "id": 33,
//                 "element": {
//                     "effects": {
//                         "sky_color": 7842047,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.9,
//                     "downfall": 1.0
//                 }
//             },
//             {
//                 "name": "minecraft:nether_wastes",
//                 "id": 34,
//                 "element": {
//                     "effects": {
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.nether.nether_wastes",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 7254527,
//                         "ambient_sound": "minecraft:ambient.nether_wastes.loop",
//                         "additions_sound": {
//                             "sound": "minecraft:ambient.nether_wastes.additions",
//                             "tick_chance": 0.0111
//                         },
//                         "water_fog_color": 329011,
//                         "fog_color": 3344392,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.nether_wastes.mood",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 0,
//                     "temperature": 2.0,
//                     "downfall": 0.0
//                 }
//             },
//             {
//                 "name": "minecraft:ocean",
//                 "id": 35,
//                 "element": {
//                     "effects": {
//                         "sky_color": 8103167,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.5,
//                     "downfall": 0.5
//                 }
//             },
//             {
//                 "name": "minecraft:old_growth_birch_forest",
//                 "id": 36,
//                 "element": {
//                     "effects": {
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.overworld.forest",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 8037887,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.6,
//                     "downfall": 0.6
//                 }
//             },
//             {
//                 "name": "minecraft:old_growth_pine_taiga",
//                 "id": 37,
//                 "element": {
//                     "effects": {
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.overworld.old_growth_taiga",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 8168447,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.3,
//                     "downfall": 0.8
//                 }
//             },
//             {
//                 "name": "minecraft:old_growth_spruce_taiga",
//                 "id": 38,
//                 "element": {
//                     "effects": {
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.overworld.old_growth_taiga",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 8233983,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.25,
//                     "downfall": 0.8
//                 }
//             },
//             {
//                 "name": "minecraft:plains",
//                 "id": 39,
//                 "element": {
//                     "effects": {
//                         "sky_color": 7907327,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.8,
//                     "downfall": 0.4
//                 }
//             },
//             {
//                 "name": "minecraft:river",
//                 "id": 40,
//                 "element": {
//                     "effects": {
//                         "sky_color": 8103167,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.5,
//                     "downfall": 0.5
//                 }
//             },
//             {
//                 "name": "minecraft:savanna",
//                 "id": 41,
//                 "element": {
//                     "effects": {
//                         "sky_color": 7254527,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 0,
//                     "temperature": 2.0,
//                     "downfall": 0.0
//                 }
//             },
//             {
//                 "name": "minecraft:savanna_plateau",
//                 "id": 42,
//                 "element": {
//                     "effects": {
//                         "sky_color": 7254527,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 0,
//                     "temperature": 2.0,
//                     "downfall": 0.0
//                 }
//             },
//             {
//                 "name": "minecraft:small_end_islands",
//                 "id": 43,
//                 "element": {
//                     "effects": {
//                         "sky_color": 0,
//                         "water_fog_color": 329011,
//                         "fog_color": 10518688,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 0,
//                     "temperature": 0.5,
//                     "downfall": 0.5
//                 }
//             },
//             {
//                 "name": "minecraft:snowy_beach",
//                 "id": 44,
//                 "element": {
//                     "effects": {
//                         "sky_color": 8364543,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4020182,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.05,
//                     "downfall": 0.3
//                 }
//             },
//             {
//                 "name": "minecraft:snowy_plains",
//                 "id": 45,
//                 "element": {
//                     "effects": {
//                         "sky_color": 8364543,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.0,
//                     "downfall": 0.5
//                 }
//             },
//             {
//                 "name": "minecraft:snowy_slopes",
//                 "id": 46,
//                 "element": {
//                     "effects": {
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.overworld.snowy_slopes",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 8560639,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": -0.3,
//                     "downfall": 0.9
//                 }
//             },
//             {
//                 "name": "minecraft:snowy_taiga",
//                 "id": 47,
//                 "element": {
//                     "effects": {
//                         "sky_color": 8625919,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4020182,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": -0.5,
//                     "downfall": 0.4
//                 }
//             },
//             {
//                 "name": "minecraft:soul_sand_valley",
//                 "id": 48,
//                 "element": {
//                     "effects": {
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.nether.soul_sand_valley",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 7254527,
//                         "ambient_sound": "minecraft:ambient.soul_sand_valley.loop",
//                         "additions_sound": {
//                             "sound": "minecraft:ambient.soul_sand_valley.additions",
//                             "tick_chance": 0.0111
//                         },
//                         "particle": {
//                             "probability": 0.00625,
//                             "options": {
//                                 "type": "minecraft:ash"
//                             }
//                         },
//                         "water_fog_color": 329011,
//                         "fog_color": 1787717,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.soul_sand_valley.mood",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 0,
//                     "temperature": 2.0,
//                     "downfall": 0.0
//                 }
//             },
//             {
//                 "name": "minecraft:sparse_jungle",
//                 "id": 49,
//                 "element": {
//                     "effects": {
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.overworld.sparse_jungle",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 7842047,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.95,
//                     "downfall": 0.8
//                 }
//             },
//             {
//                 "name": "minecraft:stony_peaks",
//                 "id": 50,
//                 "element": {
//                     "effects": {
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.overworld.stony_peaks",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 7776511,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 1.0,
//                     "downfall": 0.3
//                 }
//             },
//             {
//                 "name": "minecraft:stony_shore",
//                 "id": 51,
//                 "element": {
//                     "effects": {
//                         "sky_color": 8233727,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.2,
//                     "downfall": 0.3
//                 }
//             },
//             {
//                 "name": "minecraft:sunflower_plains",
//                 "id": 52,
//                 "element": {
//                     "effects": {
//                         "sky_color": 7907327,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.8,
//                     "downfall": 0.4
//                 }
//             },
//             {
//                 "name": "minecraft:swamp",
//                 "id": 53,
//                 "element": {
//                     "effects": {
//                         "grass_color_modifier": "swamp",
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.overworld.swamp",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 7907327,
//                         "foliage_color": 6975545,
//                         "water_fog_color": 2302743,
//                         "fog_color": 12638463,
//                         "water_color": 6388580,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.8,
//                     "downfall": 0.9
//                 }
//             },
//             {
//                 "name": "minecraft:taiga",
//                 "id": 54,
//                 "element": {
//                     "effects": {
//                         "sky_color": 8233983,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.25,
//                     "downfall": 0.8
//                 }
//             },
//             {
//                 "name": "minecraft:the_end",
//                 "id": 55,
//                 "element": {
//                     "effects": {
//                         "sky_color": 0,
//                         "water_fog_color": 329011,
//                         "fog_color": 10518688,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 0,
//                     "temperature": 0.5,
//                     "downfall": 0.5
//                 }
//             },
//             {
//                 "name": "minecraft:the_void",
//                 "id": 56,
//                 "element": {
//                     "effects": {
//                         "sky_color": 8103167,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 0,
//                     "temperature": 0.5,
//                     "downfall": 0.5
//                 }
//             },
//             {
//                 "name": "minecraft:warm_ocean",
//                 "id": 57,
//                 "element": {
//                     "effects": {
//                         "sky_color": 8103167,
//                         "water_fog_color": 270131,
//                         "fog_color": 12638463,
//                         "water_color": 4445678,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.5,
//                     "downfall": 0.5
//                 }
//             },
//             {
//                 "name": "minecraft:warped_forest",
//                 "id": 58,
//                 "element": {
//                     "effects": {
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.nether.warped_forest",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 7254527,
//                         "ambient_sound": "minecraft:ambient.warped_forest.loop",
//                         "additions_sound": {
//                             "sound": "minecraft:ambient.warped_forest.additions",
//                             "tick_chance": 0.0111
//                         },
//                         "particle": {
//                             "probability": 0.01428,
//                             "options": {
//                                 "type": "minecraft:warped_spore"
//                             }
//                         },
//                         "water_fog_color": 329011,
//                         "fog_color": 1705242,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.warped_forest.mood",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 0,
//                     "temperature": 2.0,
//                     "downfall": 0.0
//                 }
//             },
//             {
//                 "name": "minecraft:windswept_forest",
//                 "id": 59,
//                 "element": {
//                     "effects": {
//                         "sky_color": 8233727,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.2,
//                     "downfall": 0.3
//                 }
//             },
//             {
//                 "name": "minecraft:windswept_gravelly_hills",
//                 "id": 60,
//                 "element": {
//                     "effects": {
//                         "sky_color": 8233727,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.2,
//                     "downfall": 0.3
//                 }
//             },
//             {
//                 "name": "minecraft:windswept_hills",
//                 "id": 61,
//                 "element": {
//                     "effects": {
//                         "sky_color": 8233727,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 1,
//                     "temperature": 0.2,
//                     "downfall": 0.3
//                 }
//             },
//             {
//                 "name": "minecraft:windswept_savanna",
//                 "id": 62,
//                 "element": {
//                     "effects": {
//                         "sky_color": 7254527,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 0,
//                     "temperature": 2.0,
//                     "downfall": 0.0
//                 }
//             },
//             {
//                 "name": "minecraft:wooded_badlands",
//                 "id": 63,
//                 "element": {
//                     "effects": {
//                         "music": {
//                             "replace_current_music": 0,
//                             "max_delay": 24000,
//                             "sound": "minecraft:music.overworld.badlands",
//                             "min_delay": 12000
//                         },
//                         "sky_color": 7254527,
//                         "grass_color": 9470285,
//                         "foliage_color": 10387789,
//                         "water_fog_color": 329011,
//                         "fog_color": 12638463,
//                         "water_color": 4159204,
//                         "mood_sound": {
//                             "tick_delay": 6000,
//                             "offset": 2.0,
//                             "sound": "minecraft:ambient.cave",
//                             "block_search_extent": 8
//                         }
//                     },
//                     "has_precipitation": 0,
//                     "temperature": 2.0,
//                     "downfall": 0.0
//                 }
//             }
//         ]
//     }
// }
