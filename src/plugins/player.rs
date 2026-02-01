use crate::systems::{
    check_collisions, check_in_bounds, controls, gravity_system, respawn_on_endgame, spawn_player,
};
use bevy::{
    app::{App, FixedUpdate, Plugin, Startup, Update},
    ecs::schedule::IntoScheduleConfigs,
};

#[cfg(feature = "debug")]
use bevy::ecs::system::ResMut;
#[cfg(feature = "debug")]
use bevy::gizmos::config::{DefaultGizmoConfigGroup, GizmoConfigStore};
#[cfg(feature = "debug")]
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(
            FixedUpdate,
            (gravity_system, check_in_bounds, check_collisions).chain(),
        );
        app.add_systems(Update, controls);
        app.add_observer(respawn_on_endgame);

        #[cfg(feature = "debug")]
        app.add_systems(Startup, gizmos);
        #[cfg(feature = "debug")]
        app.add_plugins(EguiPlugin::default());
        #[cfg(feature = "debug")]
        app.add_plugins(WorldInspectorPlugin::new());
    }
}

#[cfg(feature = "debug")]
fn gizmos(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.enabled = true;
}
