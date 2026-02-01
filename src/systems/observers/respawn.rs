use bevy::ecs::{
    entity::Entity,
    observer::On,
    query::With,
    system::{Commands, Single},
};

use crate::{
    components::{Player, Velocity},
    config::SPAWN_TRANSFORM,
    events::EndGame,
};

pub fn respawn_on_endgame(
    _: On<EndGame>,
    mut commands: Commands,
    player: Single<Entity, With<Player>>,
) {
    commands
        .entity(*player)
        .insert((SPAWN_TRANSFORM, Velocity(0f32)));
}
