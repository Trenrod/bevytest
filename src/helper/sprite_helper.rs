use bevy::prelude::*;

use crate::atlas::atlas_sprite_details::AtlasSpriteDetails;

pub fn get_textureatlas_from_texture_and_spritedetails(
    sprite_details: AtlasSpriteDetails,
    texture: Handle<Image>,
) -> TextureAtlas {
    let texture_atlas = TextureAtlas::from_grid(
        texture,
        sprite_details.single_sprite_dimension.get_vec2(),
        sprite_details.columns,
        sprite_details.rows,
    );
    texture_atlas
}
