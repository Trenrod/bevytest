use bevy::prelude::Component;

/// Name of any entity form player to enemies
#[derive(Component)]
pub struct Name {
    pub name: String,
}
