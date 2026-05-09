use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    text::{TextFont, TextColor},
    ui::{AlignSelf, Node, PositionType},
};

#[derive(Component)]
pub struct FpsText;

pub fn dash_fps_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bold: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    let medium: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                align_self: AlignSelf::FlexEnd,
                ..default()
            },
            Text::new("FPS: "),
            TextFont {
                font: bold.clone(),
                font_size: 60.0,
                ..default()
            },
            FpsText,
        ))
        .with_child((
            TextSpan::default(),
            TextFont {
                font: medium.clone(),
                font_size: 60.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.84, 0.0)),
        ));
}

pub fn dash_fps_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.0 = format!("FPS: {value:.1}");
            }
        }
    }
}
