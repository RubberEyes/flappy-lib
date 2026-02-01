use bevy::ecs::{observer::On, system::ResMut};

use crate::{events::ScorePoint, resources::Score};

pub fn point_scored(_trigger: On<ScorePoint>, mut score: ResMut<Score>) {
    score.0 += 1;
}
