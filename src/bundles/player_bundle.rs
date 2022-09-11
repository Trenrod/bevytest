use bevy::{prelude::Bundle, sprite::SpriteSheetBundle};

use crate::components::{
    health::{Health, HealthRegeneration},
    markers::Player,
    movement::Movement,
};

use super::animation_bundle::AnimationBundle;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub health: Health,
    pub health_regeneration: HealthRegeneration,
    pub movement: Movement,
    pub marker: Player,
    #[bundle]
    pub animation: AnimationBundle,
    #[bundle]
    pub sprite: SpriteSheetBundle,
}
