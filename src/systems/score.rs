use bevy::{
    ecs::{
        query::With,
        system::{Query, Res},
    },
    ui::widget::Text,
};

use crate::{components::ScoreText, resources::Score};

pub fn score_update(mut query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    for mut span in &mut query {
        span.0 = score.0.to_string();
    }
}
