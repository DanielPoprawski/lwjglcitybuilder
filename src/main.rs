use std::f32::consts::PI;

use bevy::{
    color::palettes::css::*,
    core_pipeline::bloom::Bloom,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    input::mouse::MouseMotion,
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
    window::CursorGrabMode,
};

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "My City Builder".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, startup)
        .add_systems(Update, update)
        .add_systems(Update, handle_input)
        // .add_systems(Update, handle_light)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .run();
}

#[derive(Component, Default)]
struct CameraController {
    pitch: f32,
    yaw: f32,
    sensitivity: f32,
    velocity: Vec2,
    smoothing: f32,
    speed: f32,
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera3d {
            ..Default::default()
        },
        CameraController {
            pitch: 0.0,
            yaw: 0.0,
            sensitivity: 0.002,
            velocity: Vec2::ZERO,
            smoothing: 0.15,
            speed: 5.0,
        },
        Bloom {
            intensity: 0.3,
            low_frequency_boost: 0.7,
            low_frequency_boost_curvature: 0.95,
            high_pass_frequency: 1.0,
            composite_mode: bevy::core_pipeline::bloom::BloomCompositeMode::Additive,
            ..default()
        },
    ));
    commands.spawn(SceneRoot(
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("map.glb")),
    ));
    commands.insert_resource(AmbientLight {
        color: WHITE.into(),
        brightness: 10.,
        ..default()
    });
    commands.spawn((
        // PointLight {
        //     intensity: 100_000_000.0,
        //     color: WHITE.into(),
        //     shadows_enabled: true,
        //     ..default()
        // },
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 200.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 500.0,
            ..default()
        }
        .build(),
    ));
}
fn update() {}

// fn handle_light(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     query: Query<&mut CameraController>,
//     mut counter: Local<f32>,
// ) {
//     for mut camera in query {
//         if *counter == 0.0 {
//             *counter = 0.85;
//         }
//         if keyboard_input.just_pressed(KeyCode::ArrowUp) {
//             *counter += 0.05;
//         }
//         if keyboard_input.just_pressed(KeyCode::ArrowDown) {
//             *counter -= 0.05;
//         }
//         *counter = counter.clamp(0.01, 0.9999);
//         // camera.smoothing = *counter;
//         // println!("{}", camera.smoothing);
//     }
// }

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut mouse_movement: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera3d>>,
    // light: ResMut<AmbientLight>,
    time: Res<Time>,
    mut windows: Query<&mut Window>,
) {
    for (mut transform, mut camera_controller) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            let forward: Vec3 = *transform.forward();
            transform.translation += forward * time.delta_secs() * camera_controller.speed;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            let left: Vec3 = *transform.left();
            transform.translation += left * time.delta_secs() * camera_controller.speed;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            let back: Vec3 = *transform.back();
            transform.translation += back * time.delta_secs() * camera_controller.speed;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            let right: Vec3 = *transform.right();
            transform.translation += right * time.delta_secs() * camera_controller.speed;
        }
        if keyboard_input.pressed(KeyCode::Space) {
            let up: Vec3 = *transform.up();
            transform.translation += up * time.delta_secs() * camera_controller.speed;
        }
        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            let down: Vec3 = *transform.down();
            transform.translation += down * time.delta_secs() * camera_controller.speed;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            // light.brightness += 0.01;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            // light.brightness -= 0.01;
        }
        if mouse_input.pressed(MouseButton::Right) {
            if let Ok(mut window) = windows.single_mut() {
                window.cursor_options.grab_mode = CursorGrabMode::Locked;
                window.cursor_options.visible = false;
            }
            let mut cumulative_movement = Vec2::ZERO;
            for event in mouse_movement.read() {
                cumulative_movement += event.delta;
            }
            camera_controller.velocity = camera_controller.velocity * camera_controller.smoothing
                + cumulative_movement * (1.0 - camera_controller.smoothing);
            camera_controller.yaw -= camera_controller.velocity.x * camera_controller.sensitivity;
            camera_controller.pitch -= camera_controller.velocity.y * camera_controller.sensitivity;
        } else {
            mouse_movement.clear();
            camera_controller.velocity *= 0.2;
            if let Ok(mut window) = windows.single_mut() {
                window.cursor_options.grab_mode = CursorGrabMode::None;
                window.cursor_options.visible = true;
            }
        }
        let yaw_quat = Quat::from_rotation_y(camera_controller.yaw);
        let pitch_quat = Quat::from_rotation_x(camera_controller.pitch);
        transform.rotation = yaw_quat * pitch_quat;
    }
}
