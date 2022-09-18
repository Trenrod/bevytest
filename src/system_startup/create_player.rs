use bevy::{
    prelude::{Assets, Commands, Quat, Res, ResMut, Transform, Vec2, Vec3},
    sprite::{SpriteSheetBundle, TextureAtlas},
};
use bevy_rapier2d::prelude::{ActiveCollisionTypes, ActiveEvents, Collider};

use crate::{
    bundles::{animation_bundle::AnimationBundle, player_bundle::PlayerBundle},
    components::{
        health::{Health, HealthRegeneration},
        markers::PlayerMarker,
        movement::Movement,
    },
    helper::animation::get_default_animation_timer,
    statics::{PLAYER_CONFIG_SWORDFIGHTER, PLAYER_Z_LAYER},
    ui::loading::ui_loading::InitialisationFlags,
    GameAssets,
};

fn create_player_bundle(
    game_assets: Res<GameAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) -> PlayerBundle {
    // Load texture resources
    let texture_atlas = TextureAtlas::from_grid(
        game_assets.player_swordfighter.clone(),
        PLAYER_CONFIG_SWORDFIGHTER
            .sprite_details
            .single_sprite_dimension
            .get_vec2(),
        PLAYER_CONFIG_SWORDFIGHTER.sprite_details.columns,
        PLAYER_CONFIG_SWORDFIGHTER.sprite_details.rows,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // let texture_atlas_handle = texture_atlases.get_handle();
    let player_bundle = PlayerBundle {
        health: Health { hp: 100.0 },
        health_regeneration: HealthRegeneration {
            regeneration_rate_per_second: 1.0,
        },
        movement: Movement {
            velocity_unit_vector: Vec2::new(0.0, 0.0),
            speed: 100.0,
        },
        marker: PlayerMarker,
        animation: AnimationBundle {
            frames: PLAYER_CONFIG_SWORDFIGHTER.animation_frames,
            timer: get_default_animation_timer(),
        },
        sprite: SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                rotation: Quat::IDENTITY,
                scale: Vec3::splat(1.0),
                translation: Vec3::new(0.0, 0.0, PLAYER_Z_LAYER),
            },
            ..Default::default()
        },
        collider: (Collider::compound(vec![(
            Vec2::new(0.0, -20.0),
            0.0,
            Collider::cuboid(15.0, 30.0),
        )])),
    };
    player_bundle
}

/// Creates a player controlled component bundle
pub fn spawn_player(
    mut commands: Commands,
    mut init_state: ResMut<InitialisationFlags>,
    game_assets: Res<GameAssets>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    println!("Spawn player");
    let player_bundle = create_player_bundle(game_assets, texture_atlases);
    commands
        .spawn_bundle(player_bundle)
        .insert(ActiveCollisionTypes::STATIC_STATIC)
        .insert(ActiveEvents::COLLISION_EVENTS);
    init_state.player_created = true;
}
