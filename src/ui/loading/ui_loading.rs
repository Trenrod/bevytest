use bevy::prelude::*;
use iyes_loopless::prelude::*;
use std::time::Duration;

use crate::{game_states::GameStates, statics::DEFAULT_FONT_PATH};

#[derive(Component)]
pub struct SplashScreenTextLoadingAssets;

#[derive(Component)]
pub struct SplashScreenTextGenerateWorld;

#[derive(Component)]
pub struct UILoadingTextTimer {
    timer: Timer,
    iteration: usize,
}

#[derive(Component)]
pub struct LoadingUIMarker;

#[derive(Default, Clone, Debug)]
pub struct InitialisationFlags {
    pub assets_loaded: bool,
    pub terrain_created: bool,
    pub player_created: bool,
}

impl InitialisationFlags {
    pub fn init() -> Self {
        InitialisationFlags {
            assets_loaded: false,
            terrain_created: false,
            player_created: false,
        }
    }
}

pub fn update_loading_screen(
    mut commands: Commands,
    timer: Res<Time>,
    init_state: Res<InitialisationFlags>,
    mut query: Query<(
        &mut Text,
        &mut UILoadingTextTimer,
        Option<&SplashScreenTextGenerateWorld>,
        Option<&SplashScreenTextLoadingAssets>,
    )>,
) {
    for (mut text, mut ui_timer, marker_generate_world, marger_loading_assets) in query.iter_mut() {
        ui_timer.timer.tick(timer.delta());

        if ui_timer.timer.just_finished() {
            ui_timer.iteration += 1;
            ui_timer.iteration %= 5;
        }

        if marger_loading_assets.is_some() {
            if init_state.assets_loaded {
                text.sections[0].value = "Loading assets: done.".to_string();
            } else {
                text.sections[0].value =
                    format!("Loadig assts: pending{}", ".".repeat(ui_timer.iteration));
            }
        }

        if marker_generate_world.is_some() {
            if init_state.terrain_created {
                text.sections[0].value = "Generating world: done".to_string();
            } else {
                text.sections[0].value = format!(
                    "Generating world: pending{}",
                    ".".repeat(ui_timer.iteration)
                );
            }
        }
    }

    if init_state.assets_loaded && init_state.terrain_created && init_state.player_created {
        commands.insert_resource(NextState(GameStates::InGame));
    }
}

pub fn init_loading_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add text to show while loading
    let textitem_style = Style {
        align_self: AlignSelf::FlexStart,
        ..Default::default()
    };

    let textitem_main_textstyle = TextStyle {
        font: asset_server.load(DEFAULT_FONT_PATH),
        font_size: 20.0,
        color: Color::WHITE,
    };

    let textitem_textstyle = TextStyle {
        font: asset_server.load(DEFAULT_FONT_PATH),
        font_size: 15.0,
        color: Color::WHITE,
    };

    commands
        .spawn_bundle(
            TextBundle::from_section(
                "",
                TextStyle {
                    font: asset_server.load(DEFAULT_FONT_PATH),
                    font_size: 10.0,
                    color: Color::WHITE,
                },
            )
            .with_style(Style {
                flex_direction: FlexDirection::ColumnReverse,
                align_content: AlignContent::FlexEnd,
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            }),
        )
        .insert(LoadingUIMarker)
        .add_children(|parent| {
            parent.spawn_bundle(
                TextBundle::from_section("Generate world", textitem_main_textstyle.clone())
                    .with_style(textitem_style.clone()),
            );
            parent
                .spawn_bundle(
                    TextBundle::from_section(
                        "Loadig assts: ...pending",
                        textitem_textstyle.clone(),
                    )
                    .with_style(textitem_style.clone()),
                )
                .insert(SplashScreenTextLoadingAssets)
                .insert(UILoadingTextTimer {
                    timer: Timer::new(Duration::from_millis(500), true),
                    iteration: 0,
                });
            parent
                .spawn_bundle(
                    TextBundle::from_section("Generating world: ...pending", textitem_textstyle)
                        .with_style(textitem_style),
                )
                .insert(SplashScreenTextGenerateWorld)
                .insert(UILoadingTextTimer {
                    timer: Timer::new(Duration::from_millis(500), true),
                    iteration: 0,
                });
        });
}

pub fn update_loading_state(mut init_state: ResMut<InitialisationFlags>) {
    init_state.assets_loaded = true;
}

pub fn remote_loading_ui(mut commands: Commands, query: Query<Entity, With<LoadingUIMarker>>) {
    println!("Despawn LoadingUIMarker");
    let entity = query.single();
    commands.entity(entity).despawn();
}
