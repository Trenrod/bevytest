use bevy::{prelude::Bundle, sprite::SpriteSheetBundle};

use crate::components::{
    health::{Health, HealthRegeneration},
    markers::PlayerMarker,
    movement::Movement,
};

use super::animation_bundle::AnimationBundle;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub health: Health,
    pub health_regeneration: HealthRegeneration,
    pub movement: Movement,
    pub marker: PlayerMarker,
    #[bundle]
    pub animation: AnimationBundle,
    #[bundle]
    pub sprite: SpriteSheetBundle,
}
