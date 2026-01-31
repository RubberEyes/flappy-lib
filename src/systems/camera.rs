use bevy::{
    camera::{Camera2d, OrthographicProjection, Projection},
    ecs::system::Commands,
};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::camera::ScalingMode::AutoMax {
                max_width: 480f32,
                max_height: 270f32,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}
