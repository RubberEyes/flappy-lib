use bevy::{
    asset::AssetServer,
    color::palettes::tailwind::GRAY_700,
    ecs::{bundle::Bundle, name::Name, system::Res},
    sprite::Sprite,
    transform::components::Transform,
};

use crate::{
    components::{Gravity, Player, Velocity},
    config::{PLAYER_SIZE, SPAWN_TRANSFORM},
};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: Name,
    pub sprite: Sprite,
    pub transform: Transform,
    pub player: Player,
    pub gravity: Gravity,
    pub velocity: Velocity,
}

impl PlayerBundle {
    pub fn new(asset_server: Res<AssetServer>) -> Self {
        Self {
            name: Name::new("Flappy Player"),
            sprite: Sprite {
                image: asset_server.load("bevy-bird.png"),
                custom_size: Some(bevy::math::Vec2::splat(PLAYER_SIZE)),
                color: GRAY_700.into(),
                ..Default::default()
            },
            transform: SPAWN_TRANSFORM,
            player: Player,
            gravity: Gravity(1000f32),
            velocity: Velocity(600f32),
        }
    }
}
