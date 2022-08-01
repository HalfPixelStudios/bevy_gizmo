use bevy::{prelude::*, reflect::TypeUuid, render::render_resource::*, sprite::Material2d};

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "f064b354-c877-47a6-81d8-4ff1203a6034"]
pub struct LineMaterial {
    #[uniform(0)]
    pub color: Color,
}

impl Material2d for LineMaterial {
    fn fragment_shader() -> ShaderRef {
        "line_renderer.wgsl".into()
    }
}
