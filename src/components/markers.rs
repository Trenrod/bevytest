use bevy::prelude::Component;

/// Player, controlled by local user
#[derive(Component)]
pub struct Player;

/// RemotePlayer, controlled by network user
#[derive(Component)]
pub struct RemotePlayer;

/// Enemy controller by bots
#[derive(Component)]
pub struct Enemy;

/// Entity controller by bots
#[derive(Component)]
pub struct NPC;
