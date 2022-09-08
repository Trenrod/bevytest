use bevy::prelude::*;

use crate::components::{markers::Player, movement::Movement};

pub fn keyboard_input(keys: Res<Input<KeyCode>>, mut query: Query<(&mut Movement, With<Player>)>) {
    let mut movement = query.single_mut();

    if keys.pressed(KeyCode::W) {
        movement.0.velocity_unit_vector.y = 1.0;
    } else if keys.pressed(KeyCode::S) {
        movement.0.velocity_unit_vector.y = -1.0;
    } else {
        movement.0.velocity_unit_vector.y = 0.0;
    }

    if keys.pressed(KeyCode::A) {
        movement.0.velocity_unit_vector.x = -1.0;
    } else if keys.pressed(KeyCode::D) {
        movement.0.velocity_unit_vector.x = 1.0;
    } else {
        movement.0.velocity_unit_vector.x = 0.0;
    }
}
