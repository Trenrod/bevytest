use bevy::prelude::*;
use bevy_rapier2d::na::{Point2, Vector2};

use crate::{
    components::{markers::PlayerMarker, movement::Movement},
    events::player_action_event::{ActionType, PlayerActionEvent},
};

pub fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut ev_player_action_event: EventWriter<PlayerActionEvent>,
    mut query: Query<(Entity, &mut Movement, With<PlayerMarker>)>,
) {
    let (entity, mut movement, ()) = query.single_mut();

    if keys.pressed(KeyCode::W) {
        movement.velocity_unit_vector.y = 1.0;
    } else if keys.pressed(KeyCode::S) {
        movement.velocity_unit_vector.y = -1.0;
    } else {
        movement.velocity_unit_vector.y = 0.0;
    }

    if keys.pressed(KeyCode::A) {
        movement.velocity_unit_vector.x = -1.0;
    } else if keys.pressed(KeyCode::D) {
        movement.velocity_unit_vector.x = 1.0;
    } else {
        movement.velocity_unit_vector.x = 0.0;
    }

    if movement.velocity_unit_vector.x != 0.0 || movement.velocity_unit_vector.y != 0.0 {
        let norm_vec = Vector2::new(
            movement.velocity_unit_vector.x,
            movement.velocity_unit_vector.y,
        )
        .normalize();

        movement.velocity_unit_vector.x = norm_vec.x;
        movement.velocity_unit_vector.y = norm_vec.y;
    }

    // Actions
    if keys.pressed(KeyCode::Space) {
        ev_player_action_event.send(PlayerActionEvent {
            actionType: ActionType::ActionAttack,
            entity: entity,
        });
    }
}
