use super::Gravity;
use super::Velocity;
use bevy::ecs::component::Component;

#[derive(Component, Default)]
#[require(Gravity(1000f32), Velocity)]
pub struct Player;
