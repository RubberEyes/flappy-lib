use bevy::{
    ecs::{
        entity::Entity,
        error::Result,
        query::{Or, With},
        system::{Commands, Query, Single},
    },
    log::info,
    math::{
        Vec3Swizzles,
        bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    },
    sprite::Sprite,
    transform::helper::TransformHelper,
};

use crate::{
    components::{PipeBottom, PipeGate, PipeTop, Player},
    config::PLAYER_SIZE,
    events::EndGame,
};

#[cfg(feature = "debug")]
use bevy::{color::palettes::tailwind::RED_400, gizmos::gizmos::Gizmos};

pub fn check_collisions(
    mut commands: Commands,
    player: Single<(&Sprite, Entity), With<Player>>,
    pipe_segments: Query<(&Sprite, Entity), Or<(With<PipeTop>, With<PipeBottom>)>>,
    pipe_gaps: Query<(&Sprite, Entity), With<PipeGate>>,
    #[cfg(feature = "debug")] mut gizmos: Gizmos,
    transform_helper: TransformHelper,
) -> Result<()> {
    let player_transform = transform_helper.compute_global_transform(player.1)?;
    let player_collider =
        BoundingCircle::new(player_transform.translation().xy(), PLAYER_SIZE / 2f32);

    #[cfg(feature = "debug")]
    gizmos.circle_2d(
        player_transform.translation().xy(),
        PLAYER_SIZE / 2f32,
        RED_400,
    );

    for (sprite, entity) in &pipe_segments {
        let pipe_transform = transform_helper.compute_global_transform(entity)?;
        let pipe_collider = Aabb2d::new(
            pipe_transform.translation().xy(),
            sprite.custom_size.unwrap() / 2f32,
        );
        #[cfg(feature = "debug")]
        gizmos.rect_2d(
            pipe_transform.translation().xy(),
            sprite.custom_size.unwrap(),
            RED_400,
        );
        if player_collider.intersects(&pipe_collider) {
            commands.trigger(EndGame);
        }
    }

    for (sprite, entity) in &pipe_gaps {
        let gap_transform = transform_helper.compute_global_transform(entity)?;
        let gap_collider = Aabb2d::new(
            gap_transform.translation().xy(),
            sprite.custom_size.unwrap() / 2f32,
        );
        #[cfg(feature = "debug")]
        gizmos.rect_2d(
            gap_transform.translation().xy(),
            sprite.custom_size.unwrap(),
            RED_400,
        );
        if player_collider.intersects(&gap_collider) {
            info!("Point scored!");
            commands.entity(entity).despawn();
            // commands.trigger(ScorePoint);
        }
    }

    Ok(())
}
