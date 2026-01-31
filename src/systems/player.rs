use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
};

use crate::bundles::PlayerBundle;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PlayerBundle::new(asset_server));
}
