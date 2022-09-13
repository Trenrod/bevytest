use bevy::{
    prelude::{AssetServer, Assets, Commands, Res, ResMut, Transform, Vec2, Vec3},
    sprite::{SpriteSheetBundle, TextureAtlas},
};

use crate::{
    bundles::{animation_bundle::AnimationBundle, player_bundle::PlayerBundle},
    components::{
        health::{Health, HealthRegeneration},
        markers::PlayerMarker,
        movement::Movement,
    },
    helper::animation::get_default_animation_timer,
    statics::PLAYER_CONFIG_SWORDFIGHTER,
    ui::loading::ui_loading::InitialisationFlags,
};

/// Creates a player controlled component bundle
pub fn spawn_player(
    mut commands: Commands,
    mut init_state: ResMut<InitialisationFlags>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Load texture resources
    let texture_handle = asset_server.load("swordfighter.png");

    let texture_atlas = TextureAtlas::from_grid_with_padding(
        texture_handle,
        PLAYER_CONFIG_SWORDFIGHTER
            .sprite_details
            .single_sprite_dimension
            .get_vec2(),
        PLAYER_CONFIG_SWORDFIGHTER.sprite_details.columns,
        PLAYER_CONFIG_SWORDFIGHTER.sprite_details.rows,
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 0.0),
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // let texture_atlas_handle = texture_atlases.get_handle();
    let player_bundle = PlayerBundle {
        health: Health { hp: 100.0 },
        marker: PlayerMarker,
        movement: Movement {
            velocity_unit_vector: Vec2::new(0.0, 0.0),
            speed: 100.0,
        },
        health_regeneration: HealthRegeneration {
            regeneration_rate_per_second: 1.0,
        },
        animation: AnimationBundle {
            frames: PLAYER_CONFIG_SWORDFIGHTER.animation_frames,
            timer: get_default_animation_timer(),
        },
        sprite: SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..Default::default()
        },
    };

    commands.spawn_bundle(player_bundle);
    init_state.player_created = true;
}
