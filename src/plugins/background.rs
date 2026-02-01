use bevy::{
    app::{App, Plugin, Startup},
    sprite_render::Material2dPlugin,
};

use crate::{assets::BackgroundMaterial, systems::startup};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<BackgroundMaterial>::default());
        app.add_systems(Startup, startup);
    }
}
