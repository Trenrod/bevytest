use bevy::prelude::*;

use crate::{
    components::{
        animation_frames::AnimationFrames, animation_timer::AnimationTimer, movement::Movement,
    },
    statics::AssetsAtlasAnimationFromTo,
};

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &AnimationFrames,
        &Movement,
    )>,
) {
    for (mut timer, mut sprite, anim_frames, movement) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let cur_anim_frames: AssetsAtlasAnimationFromTo;
            if movement.velocity_unit_vector.x > 0.0 {
                cur_anim_frames = anim_frames.walk_right.clone();
            } else if movement.velocity_unit_vector.x < 0.0 {
                cur_anim_frames = anim_frames.walk_left.clone();
            } else if movement.velocity_unit_vector.y > 0.0 {
                cur_anim_frames = match anim_frames.walk_up.clone() {
                    Some(anim_frame) => anim_frame.clone(),
                    None => anim_frames.walk_right.clone(),
                }
            } else if movement.velocity_unit_vector.y < 0.0 {
                cur_anim_frames = match anim_frames.walk_down.clone() {
                    Some(anim_frame) => anim_frame.clone(),
                    None => anim_frames.walk_left.clone(),
                }
            } else if movement.velocity_unit_vector.x == 0.0
                && movement.velocity_unit_vector.y == 0.0
            {
                cur_anim_frames = match anim_frames.idle.clone() {
                    Some(anim_frame) => anim_frame.clone(),
                    None => anim_frames.walk_left.clone(),
                }
            } else {
                cur_anim_frames = anim_frames.walk_right.clone();
            }

            if sprite.index < cur_anim_frames.from || sprite.index > cur_anim_frames.to {
                sprite.index = cur_anim_frames.from;
            } else if sprite.index + 1 > cur_anim_frames.to {
                sprite.index = cur_anim_frames.from;
            } else {
                sprite.index = sprite.index + 1;
            }
        }
    }
}