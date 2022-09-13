use bevy::prelude::Component;

use crate::atlas::assets_atlas_animation_from_to::AssetsAtlasAnimationFromTo;

#[derive(Component)]
pub struct AnimationFrames {
    pub walk_right: AssetsAtlasAnimationFromTo,
    pub walk_left: AssetsAtlasAnimationFromTo,
    pub walk_up: Option<AssetsAtlasAnimationFromTo>,
    pub walk_down: Option<AssetsAtlasAnimationFromTo>,
    pub idle: Option<AssetsAtlasAnimationFromTo>,
}

#[derive(Component, Clone)]
pub struct AnimationFramesAction {
    pub moveing: Option<AssetsAtlasAnimationFromTo>,
    pub action: Option<AssetsAtlasAnimationFromTo>,
}
impl AnimationFramesAction {}
