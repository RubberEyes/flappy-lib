use crate::components::{Gravity, Velocity};
use bevy::{
    ecs::system::{Query, Res},
    time::Time,
    transform::components::Transform,
};

pub fn gravity_system(
    mut transforms: Query<(&mut Transform, &mut Velocity, &Gravity)>,
    time: Res<Time>,
) {
    for (mut transform, mut velocity, gravity) in &mut transforms {
        velocity.0 -= gravity.0 * time.delta_secs();

        transform.translation.y += velocity.0 * time.delta_secs();
    }
}
