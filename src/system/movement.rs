use bevy::prelude::*;

use crate::components::movement::Movement;

pub fn movement(time: Res<Time>, mut query: Query<(&Movement, &mut Transform)>) {
    for (movement, mut transform) in query.iter_mut() {
        transform.translation.x +=
            time.delta().as_secs_f32() * movement.velocity_unit_vector.x * movement.speed;
        transform.translation.y +=
            time.delta().as_secs_f32() * movement.velocity_unit_vector.y * movement.speed;
    }
}
