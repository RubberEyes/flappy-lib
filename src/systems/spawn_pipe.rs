use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
};

use crate::bundles::PipeBundle;

pub fn spawn_pipe(mut commands: Commands, asset_server: Res<AssetServer>) {
    let gap_y_position = 0f32;
    commands.spawn(PipeBundle::new(asset_server, gap_y_position));
}
