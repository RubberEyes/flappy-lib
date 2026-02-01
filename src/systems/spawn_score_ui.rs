use bevy::ecs::system::Commands;

use crate::bundles::ScoreUi;

pub fn spawn_score_ui(mut commands: Commands) {
    commands.spawn(ScoreUi::new());
}
