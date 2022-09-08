use bevy::prelude::Component;

/// Max health
#[derive(Component)]
pub struct Health {
    pub hp: f32,
}

// Health regeneration rate per second
#[derive(Component)]
pub struct HealthRegeneration {
    pub regeneration_rate_per_second: f32,
}
