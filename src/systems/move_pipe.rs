use bevy::{
    ecs::{
        query::With,
        system::{Query, Res},
    },
    time::Time,
    transform::components::Transform,
};

use crate::{components::Pipe, config::PIPE_SPEED};

pub fn move_pipe(mut pipes: Query<&mut Transform, With<Pipe>>, time: Res<Time>) {
    for mut pipe in &mut pipes {
        pipe.translation.x -= PIPE_SPEED * time.delta_secs();
    }
}
