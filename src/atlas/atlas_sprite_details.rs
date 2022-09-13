use crate::models::pixel_dimension::PixelDimension;

#[derive(Clone)]
pub struct AtlasSpriteDetails {
    pub single_sprite_dimension: PixelDimension,
    pub columns: usize,
    pub rows: usize,
}
