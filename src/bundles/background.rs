use bevy::{
    asset::{AssetServer, Assets},
    ecs::{
        bundle::Bundle,
        system::{Res, ResMut},
    },
    image::ImageLoaderSettings,
    math::primitives::Rectangle,
    mesh::{Mesh, Mesh2d},
    sprite_render::MeshMaterial2d,
};

use crate::{assets::BackgroundMaterial, config::CANVAS_SIZE};

#[derive(Bundle)]
pub struct BackgroundBundle {
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<BackgroundMaterial>,
}

impl BackgroundBundle {
    pub fn new(
        asset_server: Res<AssetServer>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<BackgroundMaterial>>,
    ) -> Self {
        Self {
            mesh: Mesh2d(meshes.add(Rectangle::new(CANVAS_SIZE.x, CANVAS_SIZE.y))),
            material: MeshMaterial2d(materials.add(BackgroundMaterial {
                color_texture:
                    asset_server.load_with_settings("background_color_grass.png", load_settings),
            })),
        }
    }
}

fn load_settings(settings: &mut ImageLoaderSettings) {
    settings
        .sampler
        .get_or_init_descriptor()
        .set_address_mode(bevy::image::ImageAddressMode::Repeat);
}
