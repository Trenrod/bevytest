use bevy::{prelude::Bundle, sprite::SpriteSheetBundle};
use bevy_rapier2d::prelude::*;

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
    pub collider: Collider,
    #[bundle]
    pub animation: AnimationBundle,
    #[bundle]
    pub sprite: SpriteSheetBundle,
}
