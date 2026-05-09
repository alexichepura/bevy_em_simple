use bevy::{
    color::{palettes::tailwind, Color},
    ecs::system::Commands,
    light::{AmbientLight, DirectionalLight, PointLight},
    math::Quat,
    transform::components::Transform,
    utils::default,
};

pub fn light_system(mut commands: Commands) {
    // commands.spawn((
    //     PointLight {
    //         color: Color::from(tailwind::ORANGE_300),
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     Transform::from_xyz(0.0, 3.0, 0.0),
    // ));

    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform {
            rotation: Quat::from_rotation_x(-std::f32::consts::PI / 3.0),
            ..default()
        },
    ));

    commands.spawn(AmbientLight {
        color: Color::srgb(0.5, 0.5, 0.5),
        brightness: 500.0,
        ..default()
    });
}
