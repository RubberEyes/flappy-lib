use crate::systems::spawn_camera;
use bevy::app::{Plugin, Startup};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, spawn_camera);
    }
}
