use bevy::prelude::*;

use crate::components::animation_frames::AnimationFrames;

#[derive(Component)]
pub struct AnimationTimer {
    pub timer: Timer,
}

#[derive(Bundle)]
pub struct AnimationBundle {
    pub frames: AnimationFrames,
    pub timer: AnimationTimer,
}
