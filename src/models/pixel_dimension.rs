use bevy::prelude::Vec2;

#[derive(Clone, Copy)]
pub struct PixelDimension {
    pub width: u16,
    pub height: u16,
}

impl PixelDimension {
    pub fn get_vec2(self) -> Vec2 {
        Vec2::new(f32::from(self.width), f32::from(self.height))
    }
}
