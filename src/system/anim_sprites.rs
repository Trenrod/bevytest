use bevy::prelude::*;

use crate::{
    atlas::assets_atlas_animation_from_to::AssetsAtlasAnimationFromTo,
    bundles::animation_bundle::AnimationTimer,
    components::{
        animation_frames::{AnimationFrames, AnimationFramesAction},
        movement::Movement,
    },
};

pub fn animate_action_sprite(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &AnimationFramesAction,
        Option<&Movement>,
    )>,
) {
    for (entitiy, mut timer, mut sprite, anim_frames, movement) in &mut query {
        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {
            // TODO: ado define when move ends let cur_anim_frames: AssetsAtlasAnimationFromTo;
            let cur_anim_frames = anim_frames.action.clone();
            if let Some(cur_anim_frames) = cur_anim_frames {
                if sprite.index + 1 > cur_anim_frames.to {
                    commands.entity(entitiy).despawn();
                } else {
                    sprite.index += 1;
                }
            }
        }
    }
}

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
        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {
            let cur_anim_frames: AssetsAtlasAnimationFromTo;
            if movement.velocity_unit_vector.x > 0.2 {
                cur_anim_frames = anim_frames.walk_right.clone();
            } else if movement.velocity_unit_vector.x < -0.2 {
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

            if sprite.index < cur_anim_frames.from || sprite.index + 1 > cur_anim_frames.to {
                sprite.index = cur_anim_frames.from;
            } else {
                sprite.index += 1;
            }
        }
    }
}
