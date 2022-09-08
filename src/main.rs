mod atlas;
mod bundles;
mod components;
mod plugins;
mod statics;
mod system;
mod system_startup;

use bevy::{prelude::*, render::texture::ImageSettings};
use components::{
    animation_frames::AnimationFrames, animation_timer::AnimationTimer, markers::Player,
    movement::Movement,
};
use system::anim_sprites::animate_sprite;
use system_startup::{create_background::create_background, create_player::create_player};

fn system_startup(mut commands: Commands) {
    // Spawn camera
    commands.spawn_bundle(Camera2dBundle::default());
}

fn keyboard_input(keys: Res<Input<KeyCode>>, mut query: Query<(&mut Movement, With<Player>)>) {
    let mut movement = query.single_mut();

    if keys.pressed(KeyCode::W) {
        movement.0.velocity_unit_vector.y = 1.0;
    } else if keys.pressed(KeyCode::S) {
        movement.0.velocity_unit_vector.y = -1.0;
    } else {
        movement.0.velocity_unit_vector.y = 0.0;
    }

    if keys.pressed(KeyCode::A) {
        movement.0.velocity_unit_vector.x = -1.0;
    } else if keys.pressed(KeyCode::D) {
        movement.0.velocity_unit_vector.x = 1.0;
    } else {
        movement.0.velocity_unit_vector.x = 0.0;
    }
}

fn move_player(time: Res<Time>, mut query: Query<(&Movement, &mut Transform, With<Player>)>) {
    let (movement, mut transform, ()) = query.single_mut();
    transform.translation.x += time.delta().as_secs_f32() * movement.velocity_unit_vector.x * 100.0;
    transform.translation.y += time.delta().as_secs_f32() * movement.velocity_unit_vector.y * 100.0;
}

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .add_plugins(DefaultPlugins)
        .add_startup_system(system_startup)
        .add_startup_system(create_background)
        .add_startup_system(create_player)
        .add_system(keyboard_input)
        .add_system(animate_sprite)
        .add_system(move_player)
        .run();
}
