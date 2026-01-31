use bevy::{
    ecs::{
        query::With,
        system::{Commands, Single},
    },
    transform::components::Transform,
};

use crate::{
    components::Player,
    config::{CANVAS_SIZE, PLAYER_SIZE},
    events::EndGame,
};

pub fn check_in_bounds(player: Single<&Transform, With<Player>>, mut commands: Commands) {
    if player.translation.y < -CANVAS_SIZE.y / 2f32 - PLAYER_SIZE
        || player.translation.y > CANVAS_SIZE.y / 2f32 + PLAYER_SIZE
    {
        commands.trigger(EndGame);
    }
}
