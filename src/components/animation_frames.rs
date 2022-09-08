use bevy::prelude::Component;

use crate::statics::AssetsAtlasAnimationFromTo;

#[derive(Component)]
pub struct AnimationFrames {
    pub walk_right: AssetsAtlasAnimationFromTo,
    pub walk_left: AssetsAtlasAnimationFromTo,
    pub walk_up: Option<AssetsAtlasAnimationFromTo>,
    pub walk_down: Option<AssetsAtlasAnimationFromTo>,
    pub idle: Option<AssetsAtlasAnimationFromTo>,
}
