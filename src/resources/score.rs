use bevy::ecs::resource::Resource;

#[derive(Resource, Default)]
pub struct Score(pub u32);
