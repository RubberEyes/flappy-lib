use bevy::{
    asset::{AssetServer, Assets},
    ecs::system::{Commands, Res, ResMut},
    mesh::Mesh,
};

use crate::{assets::BackgroundMaterial, bundles::BackgroundBundle};

pub fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<BackgroundMaterial>>,
) {
    commands.spawn(BackgroundBundle::new(asset_server, meshes, materials));
}
