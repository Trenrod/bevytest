use crate::{
    atlas::{
        assets_atlas_animation_from_to::AssetsAtlasAnimationFromTo,
        atlas_sprite_details::AtlasSpriteDetails,
    },
    components::animation_frames::AnimationFrames,
    config::player_config::PlayerConfig,
    models::pixel_dimension::PixelDimension,
};

// Enemies

pub static DEFAULT_FONT_PATH: &str = "fonts/Milky Coffee.ttf";

// = ASSETS SPRITES
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
    asset_path: "atlases/swordfighter.png",
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

// Draw layer z

pub const BACKGROUND_Z_LAYER: f32 = 0.0;
pub const ENEMY_Z_LAYER: f32 = 0.1;
pub const PLAYER_Z_LAYER: f32 = 0.1;
pub const ACTION_Z_LAYER: f32 = 100.0;
