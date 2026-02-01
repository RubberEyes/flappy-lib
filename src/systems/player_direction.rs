use bevy::{
    ecs::{query::With, system::Single},
    math::{Quat, Vec2},
    transform::components::Transform,
};

use crate::{
    components::{Player, Velocity},
    config::PIPE_SPEED,
};

pub fn player_direction(mut player: Single<(&mut Transform, &Velocity), With<Player>>) {
    let calculated_velocity = Vec2::new(PIPE_SPEED, player.1.0);
    player.0.rotation = Quat::from_rotation_z(calculated_velocity.to_angle());
}
