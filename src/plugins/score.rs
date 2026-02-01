use crate::{
    resources::Score,
    systems::{point_scored, reset_score_on_endgame, score_update, spawn_score_ui},
};
use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::schedule::{IntoScheduleConfigs, common_conditions::resource_changed},
};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>();

        app.add_systems(Startup, spawn_score_ui);
        app.add_systems(Update, score_update.run_if(resource_changed::<Score>));

        app.add_observer(point_scored);
        app.add_observer(reset_score_on_endgame);
    }
}
