use bevy::{
    ecs::{
        query::With,
        system::{Res, Single},
    },
    input::{ButtonInput, mouse::MouseButton},
};

use crate::components::{Player, Velocity};

pub fn controls(
    mut velocity: Single<&mut Velocity, With<Player>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        velocity.0 = 400f32;
    }
}
