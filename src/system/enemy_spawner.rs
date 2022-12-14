use bevy::prelude::*;
use bevy_rapier2d::na::Point2;
use bevy_rapier2d::prelude::{ActiveCollisionTypes, ActiveEvents, Collider, RigidBody, Sensor};

use crate::bundles::animation_bundle::AnimationBundle;
use crate::bundles::enemy_bundle::EnemyBundle;
use crate::components::markers::PlayerMarker;
use crate::components::movement::Movement;
use crate::config::enemy_configs::{EnemyConfig, ENEMY_LICH};
use crate::helper::animation::get_default_animation_timer;
use crate::statics::{ENEMY_ANIMATION_SPRITE_INDEX_NOIDLE_NOATTACK, ENEMY_Z_LAYER};

use crate::components::health::{Health, HealthRegeneration};

#[derive(Component)]
pub struct EnemySpawnConfig {
    pub spawn_timer: Timer,
    pub enemy_details: EnemyConfig,
}

#[derive(Component)]
pub struct AIBotEnemyConfig;

#[derive(Component)]
pub struct EnemyMarker;

pub fn ai_bot_enemy(
    mut commands: Commands,
    mut query_enemies: Query<(&AIBotEnemyConfig, &mut Movement, &Transform), With<EnemyMarker>>,
    mut query_player: Query<(&Transform), With<PlayerMarker>>,
) {
    let player_position = query_player.single();
    for (ai_bot_config, mut movement, enemy_position) in query_enemies.iter_mut() {
        // Calculate direction vector
        let enemy_pos_point =
            Point2::new(enemy_position.translation.x, enemy_position.translation.y);
        let player_pos_point =
            Point2::new(player_position.translation.x, player_position.translation.y);
        let direction = player_pos_point - enemy_pos_point;

        // Unify direction vector and apply to enemy movement
        let union = direction.normalize();
        movement.velocity_unit_vector.x = union.x;
        movement.velocity_unit_vector.y = union.y;
    }
}

pub fn spawn_enemy_config(mut commands: Commands) {
    println!("Spawn enemy config");
    commands.spawn().insert(EnemySpawnConfig {
        enemy_details: ENEMY_LICH,
        spawn_timer: Timer::from_seconds(5.0, false),
    });
}

pub fn spawn_enemies(
    timer: Res<Time>,
    mut commands: Commands,
    mut query: Query<&mut EnemySpawnConfig>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for mut enemy_config in query.iter_mut() {
        enemy_config.spawn_timer.tick(timer.delta());

        if enemy_config.spawn_timer.just_finished() {
            println!("Spawn enemy {}", enemy_config.enemy_details.name);

            // Load texture resources
            let texture_handle = asset_server.load(enemy_config.enemy_details.asset_path);

            let texture_atlas = TextureAtlas::from_grid_with_padding(
                texture_handle,
                enemy_config
                    .enemy_details
                    .sprite_details
                    .single_sprite_dimension
                    .get_vec2(),
                enemy_config.enemy_details.sprite_details.columns,
                enemy_config.enemy_details.sprite_details.rows,
                Vec2::new(0.0, 0.0),
                Vec2::new(0.0, 0.0),
            );

            let collider = Collider::compound(vec![(
                Vec2::new(0.0, -20.0),
                0.0,
                Collider::cuboid(15.0, 30.0),
            )]);

            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            commands
                .spawn()
                .insert_bundle(EnemyBundle {
                    animation: {
                        AnimationBundle {
                            frames: ENEMY_ANIMATION_SPRITE_INDEX_NOIDLE_NOATTACK,
                            timer: get_default_animation_timer(),
                        }
                    },
                    health: Health { hp: 1000.0 },
                    health_regeneration: HealthRegeneration {
                        regeneration_rate_per_second: 5.0,
                    },
                    sprite: SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle,
                        transform: Transform {
                            rotation: Quat::IDENTITY,
                            scale: Vec3::splat(1.0),
                            translation: Vec3::new(0.0, 0.0, ENEMY_Z_LAYER),
                        },
                        ..Default::default()
                    },
                    movement: Movement {
                        velocity_unit_vector: Vec2 { x: 0.0, y: 0.0 },
                        speed: 25.0,
                    },
                    enemy_marker: EnemyMarker {},
                    ai_bot_enemy: AIBotEnemyConfig,
                    collider: collider,
                })
                .insert(ActiveCollisionTypes::STATIC_STATIC)
                .insert(ActiveEvents::COLLISION_EVENTS);
        }
    }
}
