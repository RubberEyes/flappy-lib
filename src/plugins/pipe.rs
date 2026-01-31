pub struct PipePlugin;
use bevy::app::{App, FixedUpdate, Plugin, Startup};

use crate::systems::{move_pipe, spawn_pipe};

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_pipe);
        app.add_systems(FixedUpdate, move_pipe);
    }
}
