use bevy::prelude::Vec2;

#[derive(Clone)]
pub struct AssetsAtlasAnimationFromTo {
    pub from: usize,
    pub to: usize,
}

pub static DEFAULT_FONT_PATH: &str = "fonts/Milky Coffee.ttf";

// ASSETS SPRITES
pub const ASSETS_PLAYER_SPRITE_DIMENSION: Vec2 = Vec2::new(94.0, 100.0);
pub const ASSETS_PLAYER_SPRITE_ANIMATAION_LEFT: AssetsAtlasAnimationFromTo =
    AssetsAtlasAnimationFromTo { from: 6, to: 8 };
pub const ASSETS_PLAYER_SPRITE_ANIMATAION_RIGHT: AssetsAtlasAnimationFromTo =
    AssetsAtlasAnimationFromTo { from: 9, to: 11 };
pub const ASSETS_PLAYER_SPRITE_ANIMATAION_UP: AssetsAtlasAnimationFromTo =
    AssetsAtlasAnimationFromTo { from: 12, to: 14 };
pub const ASSETS_PLAYER_SPRITE_ANIMATAION_DOWN: AssetsAtlasAnimationFromTo =
    AssetsAtlasAnimationFromTo { from: 3, to: 5 };
pub const ASSETS_PLAYER_SPRITE_ANIMATAION_IDLE: AssetsAtlasAnimationFromTo =
    AssetsAtlasAnimationFromTo { from: 0, to: 2 };
