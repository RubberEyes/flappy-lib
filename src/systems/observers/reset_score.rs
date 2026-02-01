use bevy::ecs::{observer::On, system::ResMut};

use crate::{events::EndGame, resources::Score};

pub fn reset_score_on_endgame(_: On<EndGame>, mut score: ResMut<Score>) {
    score.0 = 0;
}
