use std::time::Duration;

use bevy::prelude::*;

pub enum DamageTypes {
    TOXIC,
    FIRE,
    ICE,
}

#[derive(Component)]
pub struct Damage {
    pub damage_type: Option<DamageTypes>,
    pub damage_over_time: Option<Duration>,
    pub damage_instant: Option<f32>,
    pub damage_per_ms: Option<f32>,
}
