use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "My City Builder".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                present_mode: bevy::window::PresentMode::Immediate,
                ..default()
            }),
            ..Default::default()
        }))
        .add_systems(Update, update)
        .add_systems(Startup, startup)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .run();
}

fn startup(mut commands: Commands, mut asset_server: Res<AssetServer>) {
    commands.spawn(Camera3d {
        ..Default::default()
    });
    commands.spawn(SceneRoot(
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("map.glb")),
    ));
}
fn update() {}
fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    for mut transform in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            transform.translation += transform.forward()
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            transform += transform.left()
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            transform += transform.back()
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            transform += transform.right()
        }
    }
}
