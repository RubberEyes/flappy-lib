pub struct PipePlugin;
use std::time::Duration;

use bevy::{
    app::{App, FixedUpdate, Plugin},
    ecs::schedule::IntoScheduleConfigs,
    time::common_conditions::on_timer,
};

use crate::systems::{check_collisions, check_in_bounds, despawn_pipe, move_pipe, spawn_pipe};

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                move_pipe,
                despawn_pipe,
                spawn_pipe.run_if(on_timer(Duration::from_secs(1))),
                check_collisions.after(check_in_bounds),
            ),
        );
    }
}
