use bevy::{
    prelude::{AssetServer, Assets, Commands, Res, ResMut, Transform, Vec2, Vec3},
    sprite::{SpriteSheetBundle, TextureAtlas},
    time::Timer,
};

use crate::{
    bundles::playerbundle::PlayerBundle,
    components::{
        animation_frames::AnimationFrames,
        animation_timer::AnimationTimer,
        health::{Health, HealthRegeneration},
        markers::Player,
        movement::Movement,
    },
    statics::{
        ASSETS_PLAYER_SPRITE_ANIMATAION_DOWN, ASSETS_PLAYER_SPRITE_ANIMATAION_IDLE,
        ASSETS_PLAYER_SPRITE_ANIMATAION_LEFT, ASSETS_PLAYER_SPRITE_ANIMATAION_RIGHT,
        ASSETS_PLAYER_SPRITE_ANIMATAION_UP, ASSETS_PLAYER_SPRITE_DIMENSION,
    },
};

/// Creates a player controlled component bundle
pub fn create_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Load texture resources
    let texture_handle = asset_server.load("swordfighter.png");

    let texture_atlas = TextureAtlas::from_grid_with_padding(
        texture_handle.clone(),
        ASSETS_PLAYER_SPRITE_DIMENSION,
        3,
        7,
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 0.0),
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // let texture_atlas_handle = texture_atlases.get_handle();
    let player_bundle = PlayerBundle {
        health: Health { hp: 100.0 },
        marker: Player,
        movement: Movement {
            velocity_unit_vector: Vec2::new(0.0, 0.0),
            speed: 10.0,
        },
        health_regeneration: HealthRegeneration {
            regeneration_rate_per_second: 1.0,
        },
        animation_frames: {
            AnimationFrames {
                walk_right: ASSETS_PLAYER_SPRITE_ANIMATAION_RIGHT,
                walk_left: ASSETS_PLAYER_SPRITE_ANIMATAION_LEFT,
                walk_up: Some(ASSETS_PLAYER_SPRITE_ANIMATAION_UP),
                walk_down: Some(ASSETS_PLAYER_SPRITE_ANIMATAION_DOWN),
                idle: Some(ASSETS_PLAYER_SPRITE_ANIMATAION_IDLE),
            }
        },
        sprite: SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..Default::default()
        },
    };

    commands.spawn_bundle(player_bundle).insert(AnimationTimer {
        timer: Timer::from_seconds(0.15, true),
    });
}
