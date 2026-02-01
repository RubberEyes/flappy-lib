use bevy::{
    asset::{Asset, Handle},
    image::Image,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    sprite_render::Material2d,
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BackgroundMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Handle<Image>,
}

impl Material2d for BackgroundMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "background.wgsl".into()
    }
}
