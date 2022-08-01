#![allow(unused)]
#![allow(dead_code)]
mod gizmo;
mod render;

use bevy::{prelude::*, sprite::Material2dPlugin};
use render::LineMaterial;

fn main() {
    let window_descriptor = WindowDescriptor {
        present_mode: bevy::window::PresentMode::Fifo,
        title: "bevy_template".into(),
        ..default()
    };

    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(window_descriptor);

    app.add_plugins(DefaultPlugins);

    app.add_startup_system(debug)
        .add_startup_system(spawn_camera);

    app.add_plugin(Material2dPlugin::<LineMaterial>::default())
        .add_startup_system(setup);

    app.run();
}

fn debug() {}

fn spawn_camera(mut cmd: Commands) {
    cmd.spawn_bundle(Camera2dBundle::default());
}

fn setup(mut cmd: Commands, mut line_materials: ResMut<Assets<LineMaterial>>) {
    let mat = line_materials.add(LineMaterial { color: Color::RED });
}
