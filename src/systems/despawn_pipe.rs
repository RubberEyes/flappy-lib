use bevy::{
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, Query},
    },
    transform::components::Transform,
};

use crate::{
    components::Pipe,
    config::{CANVAS_SIZE, PIPE_SIZE},
};

pub fn despawn_pipe(mut commands: Commands, pipes: Query<(Entity, &Transform), With<Pipe>>) {
    for (entity, transform) in pipes.iter() {
        if transform.translation.x < -(CANVAS_SIZE.x / 2f32 + PIPE_SIZE.x) {
            commands.entity(entity).despawn();
        }
    }
}
