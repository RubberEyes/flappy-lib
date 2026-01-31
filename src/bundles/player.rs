use bevy::{
    asset::AssetServer,
    ecs::{bundle::Bundle, system::Res},
    sprite::Sprite,
    transform::components::Transform,
};

use crate::components::{Gravity, Player, Velocity};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub player: Player,
    pub gravity: Gravity,
    pub velocity: Velocity,
}

impl PlayerBundle {
    pub fn new(asset_server: Res<AssetServer>) -> Self {
        Self {
            sprite: Sprite {
                image: asset_server.load("bevy-bird.png"),
                custom_size: Some(bevy::math::Vec2::splat(25f32)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0f32, 0f32, 1f32),
            player: Player,
            gravity: Gravity(1000f32),
            velocity: Velocity(600f32),
        }
    }
}
