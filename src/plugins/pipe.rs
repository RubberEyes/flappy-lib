pub struct PipePlugin;
use bevy::{
    app::{App, FixedUpdate, Plugin},
    ecs::schedule::{IntoScheduleConfigs, common_conditions::run_once},
};

use crate::systems::{move_pipe, spawn_pipe};

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, ((spawn_pipe).run_if(run_once), move_pipe));
    }
}
