use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    input::mouse::MouseMotion,
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
        .add_systems(Update, (update, handle_input))
        .add_systems(Startup, startup)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .run();
}

#[derive(Component)]
struct CameraController {
    pitch: f32,
    yaw: f32,
    sensitivity: f32,
}

fn startup(mut commands: Commands, mut asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera3d {
            ..Default::default()
        },
        CameraController {
            pitch: 0.0,
            yaw: 0.0,
            sensitivity: 0.04,
        },
    ));
    commands.spawn(SceneRoot(
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("map.glb")),
    ));
}
fn update() {}
fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut mouse_movement: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Camera3d>>,
    mut camera_controller: CameraController,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            let forward: Vec3 = *transform.forward();
            transform.translation += forward * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            let left: Vec3 = *transform.left();
            transform.translation += left * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            let back: Vec3 = *transform.back();
            transform.translation += back * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            let right: Vec3 = *transform.right();
            transform.translation += right * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::Space) {
            let up: Vec3 = *transform.up();
            transform.translation += up * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            let down: Vec3 = *transform.down();
            transform.translation += down * time.delta_secs();
        }

        for &event in mouse_movement.read() {
            if mouse_input.pressed(MouseButton::Right) {
                camera_controller.pitch += event.delta.x * camera_controller.sensitivity;
                camera_controller.yaw += event.delta.y * camera_controller.sensitivity;
                // transform.rotation = Quat::from_euler(
                //     EulerRot::XYZ,
                //     event.delta.x * 0.04,
                //     event.delta.y * 0.04,
                //     0.,
                // );
            }
        }
        transform.rotation = Quat::from_euler(
            EulerRot::XYZ,
            camera_controller.pitch,
            camera_controller.yaw,
            1.,
        );
    }
}
