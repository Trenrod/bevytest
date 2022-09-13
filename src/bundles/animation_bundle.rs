use bevy::prelude::*;

use crate::components::animation_frames::{AnimationFrames, AnimationFramesAction};

#[derive(Component)]
pub struct AnimationTimer {
    pub timer: Timer,
}

#[derive(Bundle)]
pub struct AnimationBundle {
    pub frames: AnimationFrames,
    pub timer: AnimationTimer,
}

#[derive(Bundle)]
pub struct AnimationActionBundle {
    pub frames: AnimationFramesAction,
    pub timer: AnimationTimer,
}
