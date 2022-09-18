mod atlas;
mod bundles;
mod components;
mod config;
mod events;
mod game_states;
mod helper;
mod models;
mod plugins;
mod statics;
mod system;
mod system_startup;
mod ui;

use bevy_editor_pls::prelude::*;

use bevy::{
    prelude::*,
    render::texture::{self, ImageSettings},
};

use bevy_asset_loader::prelude::*;
use bevy_rapier2d::prelude::*;
use components::{markers::PlayerMarker, movement::Movement};
use config::action_config::ACTION_CONFIG_FIRE_SLASH;
use events::player_action_event::PlayerActionEvent;
use iyes_loopless::prelude::*;

use game_states::GameStates;
use system::{
    anim_sprites::{animate_action_sprite, animate_sprite},
    enemy_spawner::{ai_bot_enemy, spawn_enemies, spawn_enemy_config},
    keyboard_input::keyboard_input,
    movement::movement,
};
use system_startup::{create_background::create_background, create_player::spawn_player};
use ui::loading::ui_loading::{
    init_loading_screen, remote_loading_ui, update_loading_screen, update_loading_state,
    InitialisationFlags,
};

use crate::{
    bundles::action_bundle::create_action_bundle,
    helper::sprite_helper::get_textureatlas_from_texture_and_spritedetails,
};

fn system_startup(mut commands: Commands) {
    // Spawn camera
    commands.spawn_bundle(Camera2dBundle::default());
}

#[derive(AssetCollection)]
pub struct GameAssets {
    #[asset(path = "atlases/swordfighter.png")]
    pub player_swordfighter: Handle<Image>,
    #[asset(path = "atlases/terrain.png")]
    pub background_terrain: Handle<Image>,
    #[asset(path = "atlases/fire_slash.png")]
    pub action_fire_slash: Handle<Image>,
}

// fn set_action_bundle(
//     mut entity: EntityCommands,
//     action_config: &ActionConfig,
//     transform: &Transform,
// ) {
//     let action_bundle = ActionInstantBundle {
//         damage: Damage {
//             damage_instant: Some(10.0),
//             damage_type: None,
//             damage_over_time: None,
//             damage_per_ms: None,
//         },
//         marker: InstantActionMarker,
//         animation: AnimationActionBundle {
//             frames: action_config.animation_frames.clone(),
//             timer: get_default_animation_timer(),
//         },
//         sprite: SpriteSheetBundle {
//             transform: Transform {
//                 translation: transform.translation.clone(),
//                 ..Default::default()
//             },
//             texture_atlas
//             ..Default::default()
//         },
//     };
//     entity.insert_bundle(action_bundle);
// }

fn handle_action(
    mut commands: Commands,
    mut ev_player_action: EventReader<PlayerActionEvent>,
    player_query: Query<(&Transform, &Movement), With<PlayerMarker>>,
    game_assets: Res<GameAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for ev in ev_player_action.iter() {
        if let Ok((player_transform, player_movement)) = player_query.get(ev.entity) {
            // Dont spawn if not moving
            if player_movement.velocity_unit_vector.x == 0.0
                && player_movement.velocity_unit_vector.y == 0.0
            {
                return;
            }

            // Spawner position
            let spawn_position = Vec2::new(
                player_transform.translation.x,
                player_transform.translation.y,
            );

            let texture_atlas = get_textureatlas_from_texture_and_spritedetails(
                ACTION_CONFIG_FIRE_SLASH.sprite_details.clone(),
                game_assets.action_fire_slash.clone(),
            );
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            let action_bundle = create_action_bundle(
                texture_atlas_handle,
                &ACTION_CONFIG_FIRE_SLASH,
                &spawn_position,
                &player_movement.velocity_unit_vector,
            );
            commands.spawn_bundle(action_bundle);
        } else {
            println!("Could not find action spawner entity");
        }
    }
}

/* A system that displays the events. */
fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}

fn main() {
    App::new()
        // prevents blurry sprites
        .insert_resource(ImageSettings::default_nearest())
        // Bevy defaults
        .add_plugins(DefaultPlugins)
        // Init initialisation state
        .insert_resource(InitialisationFlags::init())
        // Bevy default plugins
        .add_plugin(EditorPlugin)
        // Player triggers action event
        .add_event::<PlayerActionEvent>()
        // Add camera
        .add_startup_system(system_startup)
        // Add physics
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        // Initial GameState state
        .add_loopless_state(GameStates::LoadingAssets)
        // General system
        .add_system(update_loading_screen.run_not_in_state(GameStates::InGame))
        // === GameStates::LoadingAssets
        .add_loading_state(
            LoadingState::new(GameStates::LoadingAssets)
                .continue_to_state(GameStates::GeneratingWorld)
                .with_collection::<GameAssets>(),
        )
        .add_enter_system(GameStates::LoadingAssets, init_loading_screen)
        // === GameStates::GeneratingWorld
        .add_enter_system(GameStates::GeneratingWorld, update_loading_state)
        .add_enter_system(GameStates::GeneratingWorld, create_background)
        .add_enter_system(GameStates::GeneratingWorld, spawn_player)
        // === GameStates::InGame
        .add_enter_system(GameStates::InGame, remote_loading_ui)
        .add_enter_system(GameStates::InGame, spawn_enemy_config)
        .add_system(movement.run_in_state(GameStates::InGame))
        .add_system(spawn_enemies.run_in_state(GameStates::InGame))
        .add_system(animate_sprite.run_in_state(GameStates::InGame))
        .add_system(animate_action_sprite.run_in_state(GameStates::InGame))
        .add_system(keyboard_input.run_in_state(GameStates::InGame))
        .add_system(ai_bot_enemy.run_in_state(GameStates::InGame))
        .add_system(display_events.run_in_state(GameStates::InGame))
        .add_system(handle_action.run_in_state(GameStates::InGame))
        .run();
}
