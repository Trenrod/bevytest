use bevy_editor_pls::controls::Action;

use crate::{
    atlas::{
        assets_atlas_animation_from_to::AssetsAtlasAnimationFromTo,
        atlas_sprite_details::AtlasSpriteDetails,
    },
    components::animation_frames::{AnimationFrames, AnimationFramesAction},
    models::pixel_dimension::PixelDimension,
};

pub struct ActionConfig {
    pub name: &'static str,
    pub asset_path: &'static str,
    pub sprite_details: AtlasSpriteDetails,
    pub animation_frames: AnimationFramesAction,
}

pub const ACTION_CONFIG_FIRE_SLASH: ActionConfig = ActionConfig {
    name: "FireSlash",
    asset_path: "atlas/fire_slash.png",
    sprite_details: AtlasSpriteDetails {
        single_sprite_dimension: PixelDimension {
            width: 128,
            height: 128,
        },
        columns: 4,
        rows: 2,
    },
    animation_frames: AnimationFramesAction {
        moveing: None,
        action: Some(AssetsAtlasAnimationFromTo { from: 0, to: 4 }),
    },
};
