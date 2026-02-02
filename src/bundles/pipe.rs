use bevy::{
    asset::AssetServer,
    camera::visibility::Visibility,
    color::Color,
    ecs::{
        bundle::Bundle,
        children,
        hierarchy::ChildOf,
        spawn::{Spawn, SpawnRelatedBundle},
        system::Res,
    },
    image::ImageLoaderSettings,
    math::Vec2,
    sprite::{BorderRect, SliceScaleMode, Sprite, SpriteImageMode, TextureSlicer},
    transform::components::Transform,
};
use derive_getters::Dissolve;

use crate::{
    components::{Pipe, PipeBottom, PipeGate, PipeTop},
    config::{CANVAS_SIZE, GAP_SIZE, PIPE_GAP_OFFSET, PIPE_SIZE},
};

#[derive(Bundle, Dissolve)]
#[bundle(ignore_from_components)]
pub struct PipeBundle {
    pub pipe: Pipe,
    pub visibility: Visibility,
    pub transform: Transform,
    pub children: SpawnRelatedBundle<
        ChildOf,
        (
            Spawn<PipeTopBundle>,
            Spawn<PipeGateBundle>,
            Spawn<PipeBottomBundle>,
        ),
    >,
}

#[derive(Bundle, Default)]
pub struct PipeTopBundle {
    pub pipe_top: PipeTop,
    pub visibility: Visibility,
    pub transform: Transform,
    pub sprite: Sprite,
}

#[derive(Bundle, Default)]
pub struct PipeGateBundle {
    pub pipe_gate: PipeGate,
    pub visibility: Visibility,
    pub transform: Transform,
    pub sprite: Sprite,
}

#[derive(Bundle, Default)]
pub struct PipeBottomBundle {
    pub pipe_bottom: PipeBottom,
    pub visibility: Visibility,
    pub transform: Transform,
    pub sprite: Sprite,
}

impl PipeBundle {
    pub fn new(asset_server: Res<AssetServer>, position: f32) -> Self {
        Self {
            pipe: Pipe,
            visibility: Visibility::Visible,
            transform: Transform::from_xyz((PIPE_SIZE.x + CANVAS_SIZE.x) / 2f32, position, 0f32),
            children: children![
                PipeTopBundle {
                    pipe_top: PipeTop,
                    visibility: Visibility::Inherited,
                    transform: Transform::from_xyz(0f32, position + PIPE_GAP_OFFSET, 0f32),
                    sprite: Sprite {
                        image: asset_server.load_with_settings("pipe.png", load_settings),
                        custom_size: Some(PIPE_SIZE),
                        image_mode: SpriteImageMode::Sliced(TextureSlicer {
                            border: BorderRect::axes(8f32, 19f32),
                            center_scale_mode: SliceScaleMode::Stretch,
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                },
                PipeGateBundle {
                    pipe_gate: PipeGate,
                    #[cfg(feature = "debug")]
                    visibility: Visibility::Inherited,
                    #[cfg(not(feature = "debug"))]
                    visibility: Visibility::Hidden,
                    transform: Transform::from_xyz(0f32, position, 0f32),
                    sprite: Sprite {
                        color: Color::WHITE,
                        custom_size: Some(Vec2::new(8f32, GAP_SIZE)),
                        ..Default::default()
                    },
                },
                PipeBottomBundle {
                    pipe_bottom: PipeBottom,
                    visibility: Visibility::Inherited,
                    transform: Transform::from_xyz(0f32, position - PIPE_GAP_OFFSET, 0f32),
                    sprite: Sprite {
                        image: asset_server.load_with_settings("pipe.png", load_settings),
                        custom_size: Some(PIPE_SIZE),
                        image_mode: SpriteImageMode::Sliced(TextureSlicer {
                            border: BorderRect::axes(8f32, 19f32),
                            center_scale_mode: SliceScaleMode::Stretch,
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                },
            ],
        }
    }
}

fn load_settings(settings: &mut ImageLoaderSettings) {
    settings
        .sampler
        .get_or_init_descriptor()
        .set_filter(bevy::image::ImageFilterMode::Nearest);
}
