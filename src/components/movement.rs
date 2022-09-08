use bevy::prelude::{Component, Vec2};

#[derive(Component)]
pub struct Movement {
    pub velocity_unit_vector: Vec2,
    pub speed: f32,
}
