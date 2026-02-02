use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::plugins::{BackgroundPlugin, CameraPlugin, PipePlugin, PlayerPlugin, ScorePlugin};

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
