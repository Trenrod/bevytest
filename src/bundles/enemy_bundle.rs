use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;

use crate::{
    components::{
        health::{Health, HealthRegeneration},
        movement::Movement,
    },
    system::enemy_spawner::{AIBotEnemyConfig, EnemyMarker},
};

use super::animation_bundle::AnimationBundle;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub health: Health,
    pub health_regeneration: HealthRegeneration,
    pub movement: Movement,
    pub enemy_marker: EnemyMarker,
    pub collider: Collider,
    pub ai_bot_enemy: AIBotEnemyConfig,
    #[bundle]
    pub animation: AnimationBundle,
    #[bundle]
    pub sprite: SpriteSheetBundle,
}
