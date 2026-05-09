use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, DefaultPlugins};
use camera::CameraPlugin;
use dash::{dash_fps_system, dash_fps_update_system};
use field::field_system;

mod camera;
mod dash;
mod field;
mod mesh;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(CameraPlugin)
        .add_systems(Startup, dash_fps_system)
        .add_systems(Startup, field_system)
        .add_systems(Update, dash_fps_update_system)
        .run();
}
