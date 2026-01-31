use bevy::{
    math::{Quat, Vec2, Vec3},
    transform::components::Transform,
};

pub const CANVAS_SIZE: Vec2 = Vec2::new(480f32, 270f32);
pub const PLAYER_SIZE: f32 = 25f32;
pub const SPAWN_TRANSFORM: Transform = Transform {
    rotation: Quat::IDENTITY,
    scale: Vec3::ONE,
    translation: Vec3 {
        x: -CANVAS_SIZE.x / 4f32,
        y: 0f32,
        z: 1f32,
    },
};

pub const GAP_SIZE: f32 = 100f32;
pub const PIPE_SIZE: Vec2 = Vec2::new(32f32, CANVAS_SIZE.y);
pub const PIPE_GAP_OFFSET: f32 = PIPE_SIZE.y / 2f32 + GAP_SIZE / 2f32;
