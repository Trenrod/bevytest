mod atlas;
mod bundles;
mod components;
mod game_states;
mod plugins;
mod statics;
mod system;
mod system_startup;
mod ui;

use bevy_editor_pls::prelude::*;

use bevy::{prelude::*, render::texture::ImageSettings};

use bevy_asset_loader::prelude::*;
use components::{markers::Player, movement::Movement};
use iyes_loopless::prelude::*;

use game_states::GameStates;
use system::{anim_sprites::animate_sprite, keyboard_input::keyboard_input};
use system_startup::{create_background::create_background, create_player::create_player};
use ui::loading::ui_loading::{
    init_loading_screen, remote_loading_ui, update_loading_screen, update_loading_state,
    InitialisationFlags,
};

fn move_player(time: Res<Time>, mut query: Query<(&Movement, &mut Transform, With<Player>)>) {
    let (movement, mut transform, ()) = query.single_mut();
    transform.translation.x += time.delta().as_secs_f32() * movement.velocity_unit_vector.x * 100.0;
    transform.translation.y += time.delta().as_secs_f32() * movement.velocity_unit_vector.y * 100.0;
}

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
}

fn main() {
    App::new()
        // Init initialisation state
        .insert_resource(InitialisationFlags::init())
        // prevents blurry sprites
        .insert_resource(ImageSettings::default_nearest())
        // Bevy defaults
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin)
        // Add camera
        .add_startup_system(system_startup)
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
        .add_enter_system(GameStates::GeneratingWorld, create_player)
        // === GameStates::InGame
        .add_enter_system(GameStates::InGame, remote_loading_ui)
        .add_system(move_player.run_in_state(GameStates::InGame))
        .add_system(animate_sprite.run_in_state(GameStates::InGame))
        .add_system(keyboard_input.run_in_state(GameStates::InGame))
        .run();
}
