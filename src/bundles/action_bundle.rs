use bevy::prelude::*;
use bevy_rapier2d::{
    na::{Point2, Vector2},
    prelude::Collider,
    rapier::prelude::ColliderBuilder,
};

use crate::{
    atlas::atlas_sprite_details::AtlasSpriteDetails,
    components::{
        damage::{Damage, DamageTypes},
        markers::InstantActionMarker,
        movement::Movement,
    },
    config::action_config::ActionConfig,
    helper::animation::{get_default_action_animation_timer, get_default_animation_timer},
    statics::ACTION_Z_LAYER,
};

use super::animation_bundle::{AnimationActionBundle, AnimationBundle};

#[derive(Component)]
pub struct ProjectileDestination {
    pub destination: Point2<u16>,
}

pub fn create_action_bundle(
    texture_atlas: Handle<TextureAtlas>,
    action_config: &ActionConfig,
    spawn_position: &Vec2,
    looking_direction: &Vec2,
) -> ActionInstantBundle {
    // Load texture resources
    let translation = Vec3::new(
        spawn_position.x + (50.0 * looking_direction.x),
        spawn_position.y + (50.0 * looking_direction.y),
        ACTION_Z_LAYER,
    );
    // let rotation = Quat::from_rotation_y((75.0_f32).to_radians());
    let rotation = Quat::from_rotation_arc_2d(Vec2::new(1.0, 0.0), looking_direction.clone());

    let point1 = Vec2::new(0.0, 30.0);
    let point2 = Vec2::new(25.0, 75.0);
    let point3 = Vec2::new(25.0, -75.0);
    let point4 = Vec2::new(0.0, -30.0);
    let points: Vec<Vec2> = vec![point1, point2, point3, point4];
    let collider = Collider::compound(vec![(
        Vec2::new(0.0, 0.0),
        0.0,
        Collider::convex_hull(&points).unwrap(),
    )]);

    // let texture_atlas_handle = texture_atlases.get_handle();
    let action_instant_bundle = ActionInstantBundle {
        damage: Damage {
            damage_type: None,
            damage_over_time: None,
            damage_instant: Some(10.0),
            damage_per_ms: None,
        },
        marker: InstantActionMarker,
        animation: AnimationActionBundle {
            frames: action_config.animation_frames.clone(),
            timer: get_default_action_animation_timer(),
        },
        sprite: SpriteSheetBundle {
            texture_atlas: texture_atlas,
            transform: Transform {
                rotation: rotation,
                scale: Vec3::splat(1.0),
                translation: translation,
            },
            ..Default::default()
        },
        collider: collider,
    };
    action_instant_bundle
}

#[derive(Bundle)]
pub struct ActionInstantBundle {
    pub damage: Damage,
    // pub movement: Movement,
    // pub destination: ProjectileDestination,
    pub collider: Collider,
    pub marker: InstantActionMarker,
    #[bundle]
    pub animation: AnimationActionBundle,
    #[bundle]
    pub sprite: SpriteSheetBundle,
}
