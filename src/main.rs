mod atlas;
mod bundles;
mod components;
mod game_states;
mod plugins;
mod statics;
mod system;
mod system_startup;

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    render::texture::ImageSettings,
};
use bevy_asset_loader::prelude::*;
use components::{markers::Player, movement::Movement};
use game_states::GameStates;
use iyes_progress::{ProgressCounter, ProgressPlugin};
use system::{anim_sprites::animate_sprite, keyboard_input::keyboard_input};
use system_startup::{
    create_background::{self, create_background},
    create_player::create_player,
};

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
struct ColorText;

fn system_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn camera
    commands.spawn_bundle(Camera2dBundle::default());
    // Text with one section
    commands
        .spawn_bundle(
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "hello\nbevy!",
                TextStyle {
                    font: asset_server.load("fonts/Milky Coffee.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
            ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::TOP_CENTER)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                align_self: AlignSelf::FlexStart,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(FpsText)
        .insert(ColorText);
}

fn move_player(time: Res<Time>, mut query: Query<(&Movement, &mut Transform, With<Player>)>) {
    let (movement, mut transform, ()) = query.single_mut();
    transform.translation.x += time.delta().as_secs_f32() * movement.velocity_unit_vector.x * 100.0;
    transform.translation.y += time.delta().as_secs_f32() * movement.velocity_unit_vector.y * 100.0;
}

#[derive(AssetCollection)]
pub struct GameAssets {
    #[asset(path = "atlases/swordfighter.png")]
    pub player_swordfighter: Handle<Image>,
    #[asset(path = "atlases/terrain.png")]
    pub background_terrain: Handle<Image>,
}

fn track_fake_long_task(time: Res<Time>, progress: Res<ProgressCounter>) {
    if time.seconds_since_startup() > 5.0 {
        info!("Long task is completed");
        progress.manually_track(true.into());
    } else {
        info!("Wait");
        progress.manually_track(false.into());
    }
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    println!("text_update_system");
    for mut text in &mut query {
        println!("update");
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                println!("select");
                // Update the value of the second section
                text.sections[0].value = format!("{average:.2}");
            }
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .add_loading_state(
            LoadingState::new(GameStates::LoadingAssets).with_collection::<GameAssets>(),
        )
        .add_state(GameStates::LoadingAssets)
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // Track progress for GameStates::LoadingAssets and once done switch to GameStates::GeneratingWorld
        .add_plugin(
            ProgressPlugin::new(GameStates::LoadingAssets).continue_to(GameStates::GeneratingWorld),
        )
        .add_system_set(
            SystemSet::on_update(GameStates::LoadingAssets)
                .with_system(track_fake_long_task)
                .with_system(text_update_system),
        )
        // Generate World
        .add_system_set(
            SystemSet::on_enter(GameStates::GeneratingWorld)
                .with_system(create_background)
                .with_system(create_player),
        )
        // Track progress for GameStates::GeneratingWorld and once done switch to GameStates::InGame
        .add_plugin(
            ProgressPlugin::new(GameStates::GeneratingWorld).continue_to(GameStates::InGame),
        )
        // Ingame updates
        .add_system_set(
            SystemSet::on_update(GameStates::InGame)
                .with_system(keyboard_input)
                .with_system(move_player.after(keyboard_input))
                .with_system(animate_sprite.before(move_player).after(keyboard_input)),
        )
        .add_startup_system(system_startup)
        .run();
}
