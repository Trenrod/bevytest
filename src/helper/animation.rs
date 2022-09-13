use bevy::time::Timer;

use crate::bundles::animation_bundle::AnimationTimer;

// Default animation timer for all animations
pub fn get_default_animation_timer() -> AnimationTimer {
    AnimationTimer {
        timer: Timer::from_seconds(0.15, true),
    }
}
