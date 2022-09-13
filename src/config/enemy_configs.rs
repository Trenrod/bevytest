use crate::{
    atlas::atlas_sprite_details::AtlasSpriteDetails,
    components::animation_frames::AnimationFrames,
    statics::{ASSETS_DEFAULT_SPRITE_DIMENSION, ENEMY_ANIMATION_SPRITE_INDEX_NOIDLE_NOATTACK},
};

pub struct EnemyConfig {
    pub name: &'static str,
    pub asset_path: &'static str,
    pub sprite_details: AtlasSpriteDetails,
    pub animation_frames: AnimationFrames,
}

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
