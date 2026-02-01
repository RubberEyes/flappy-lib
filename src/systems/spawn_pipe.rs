use bevy::{
    asset::AssetServer,
    ecs::{
        query::With,
        system::{Commands, Res, Single},
    },
};
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::GlobalRng;
use rand::Rng;

use crate::{bundles::PipeBundle, config::CANVAS_SIZE};

pub fn spawn_pipe(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut rng: Single<&mut ChaCha8Rng, With<GlobalRng>>,
) {
    let y_position = 0.2 * (rng.random::<f32>() * CANVAS_SIZE.y - CANVAS_SIZE.y / 2f32);
    commands.spawn(PipeBundle::new(asset_server, y_position).dissolve());
}
