use crate::{
    atlas::atlas_sprite_details::AtlasSpriteDetails, components::animation_frames::AnimationFrames,
};

pub struct EnemyConfig {
    pub name: &'static str,
    pub asset_path: &'static str,
    pub sprite_details: AtlasSpriteDetails,
    pub animation_frames: AnimationFrames,
}
