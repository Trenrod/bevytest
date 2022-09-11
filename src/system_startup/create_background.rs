use bevy::prelude::*;

use noise::{utils::*, Perlin};

use crate::ui::loading::ui_loading::InitialisationFlags;

#[derive(Component)]
struct Background;

/// Creates world
pub fn create_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut init_state: ResMut<InitialisationFlags>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("outside.png");
    let texture_atlas = TextureAtlas::from_grid_with_padding(
        texture_handle,
        Vec2 { x: 32.0, y: 32.0 },
        8,
        16,
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 0.0),
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let world_size_in_tiles_x = 50;
    let world_size_in_tiles_y = 50;

    let perlin = Perlin::new();
    // let clamp = Clamp::<[f64; 3]>::new(&perlin).set_bounds(0.0, 10.0);
    let noise_map = PlaneMapBuilder::new(&perlin)
        .set_size(world_size_in_tiles_x, world_size_in_tiles_y)
        .set_x_bounds(0.0, 4.0)
        .set_y_bounds(0.0, 4.0)
        .build();

    commands
        .spawn()
        .insert_bundle(SpatialBundle::default())
        .add_children(|parent| {
            for x_pos in 0..world_size_in_tiles_x {
                for y_pos in 0..world_size_in_tiles_y {
                    let world_x_pos: f32 =
                        (x_pos as f32 * 32.0) - (world_size_in_tiles_x as f32 * 32.0 / 2.0);
                    let world_y_pos: f32 =
                        (y_pos as f32 * 32.0) - (world_size_in_tiles_x as f32 * 32.0 / 2.0);

                    let mut sb = SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle.clone(),
                        transform: Transform::from_xyz(world_x_pos, world_y_pos, 0.0),
                        ..default()
                    };

                    // Change range -1 to 1 to 0 to 10
                    let org_value = noise_map.get_value(x_pos, y_pos);
                    // NewValue = (((OldValue - OldMin) * NewRange) / OldRange) + NewMin
                    let new_value = (((org_value - -1.0) * 7.0) / (2.0)) + 1.0;

                    let map_value = new_value.round() as usize;
                    sb.sprite.index = map_value;
                    parent.spawn_bundle(sb);
                }
            }
        });
    init_state.terrain_created = true;
}
