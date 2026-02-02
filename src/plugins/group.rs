use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    camera::CameraPlugin,
};

use crate::plugins::{BackgroundPlugin, PipePlugin, PlayerPlugin, ScorePlugin};

pub struct FlappyPluginGroup;

impl PluginGroup for FlappyPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CameraPlugin)
            .add(ScorePlugin)
            .add(PlayerPlugin)
            .add(PipePlugin)
            .add(BackgroundPlugin)
    }
}
