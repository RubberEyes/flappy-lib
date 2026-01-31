use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Pipe;

#[derive(Component, Default)]
pub struct PipeTop;

#[derive(Component, Default)]
pub struct PipeBottom;

#[derive(Component, Default)]
pub struct PipeGate;
