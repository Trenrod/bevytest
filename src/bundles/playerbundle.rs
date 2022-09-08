use bevy::{prelude::Bundle, sprite::SpriteSheetBundle};

use crate::components::{
    animation_frames::AnimationFrames,
    health::{Health, HealthRegeneration},
    markers::Player,
    movement::Movement,
};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub health: Health,
    pub health_regeneration: HealthRegeneration,
    pub animation_frames: AnimationFrames,
    pub movement: Movement,
    pub marker: Player,
    #[bundle]
    pub sprite: SpriteSheetBundle,
}
