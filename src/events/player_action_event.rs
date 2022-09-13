use bevy::prelude::*;

use crate::config::action_config::ActionConfig;

pub enum ActionType {
    ActionAttack,
}

pub struct PlayerActionEvent {
    pub actionType: ActionType,
    pub entity: Entity,
}
