use crate::systems::{controls, gravity_system, spawn_player};
use bevy::app::{FixedUpdate, Plugin, Startup};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(FixedUpdate, gravity_system);
        app.add_systems(FixedUpdate, controls);
    }
}
