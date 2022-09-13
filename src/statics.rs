use bevy::prelude::Vec2;

use crate::{
    atlas::{
        assets_atlas_animation_from_to::AssetsAtlasAnimationFromTo,
        atlas_sprite_details::AtlasSpriteDetails,
    },
    components::animation_frames::AnimationFrames,
    config::{enemy_configs::EnemyConfig, player_config::PlayerConfig},
    models::pixel_dimension::PixelDimension,
};

pub const ENEMY_LICH: EnemyConfig = EnemyConfig {
    name: "Lich",
    asset_path: "atlases/enemy_lich.png",
    animation_frames: ENEMY_ANIMATION_SPRITE_INDEX_NOIDLE_NOATTACK,
    sprite_details: AtlasSpriteDetails {
        single_sprite_dimension: ASSETS_DEFAULT_SPRITE_DIMENSION,
        columns: 3,
        rows: 7,
    },
};

pub const ENEMY_BROWN_WOLF: EnemyConfig = EnemyConfig {
    name: "Brown wolf",
    asset_path: "atlases/enemy_brownwolf.png",
    animation_frames: ENEMY_ANIMATION_SPRITE_INDEX_NOIDLE_NOATTACK,
    sprite_details: AtlasSpriteDetails {
        single_sprite_dimension: ASSETS_DEFAULT_SPRITE_DIMENSION,
        columns: 3,
        rows: 7,
    },
};

pub static DEFAULT_FONT_PATH: &str = "fonts/Milky Coffee.ttf";

// = ASSETS SPRITES
pub const ENEMY_SPRITE_DETAILS: AtlasSpriteDetails = AtlasSpriteDetails {
    single_sprite_dimension: PixelDimension {
        height: 100,
        width: 94,
    },
    columns: 3,
    rows: 7,
};

pub const ASSETS_DEFAULT_SPRITE_DIMENSION: PixelDimension = PixelDimension {
    width: 94,
    height: 100,
};

// == Enemy
pub const ENEMY_ANIMATION_SPRITE_INDEX_NOIDLE_NOATTACK: AnimationFrames = AnimationFrames {
    walk_right: AssetsAtlasAnimationFromTo { from: 6, to: 8 },
    walk_left: AssetsAtlasAnimationFromTo { from: 3, to: 5 },
    walk_up: Some(AssetsAtlasAnimationFromTo { from: 9, to: 11 }),
    walk_down: Some(AssetsAtlasAnimationFromTo { from: 0, to: 2 }),
    idle: None,
};

// == Player
pub const PLAYER_CONFIG_SWORDFIGHTER: PlayerConfig = PlayerConfig {
    name: "Swordfighter",
    asset_path: "atlases/sordfighter.png",
    sprite_details: AtlasSpriteDetails {
        single_sprite_dimension: PixelDimension {
            height: 100,
            width: 94,
        },
        columns: 3,
        rows: 7,
    },
    animation_frames: PLAYER_ANIMATION_SPRITE_INDEX,
};

pub const PLAYER_ANIMATION_SPRITE_INDEX: AnimationFrames = AnimationFrames {
    walk_right: AssetsAtlasAnimationFromTo { from: 9, to: 11 },
    walk_left: AssetsAtlasAnimationFromTo { from: 6, to: 8 },
    walk_up: Some(AssetsAtlasAnimationFromTo { from: 12, to: 14 }),
    walk_down: Some(AssetsAtlasAnimationFromTo { from: 3, to: 5 }),
    idle: Some(AssetsAtlasAnimationFromTo { from: 0, to: 2 }),
};
