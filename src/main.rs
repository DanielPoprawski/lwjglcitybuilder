use bevy::{
    asset::RenderAssetUsages,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use std::{
    f32::consts::PI,
    f64::{self, consts::E},
};

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;
const SPEED: f32 = 300.;
const BOID_COUNT: u8 = 5;
const BOID_RADIUS: f32 = 500.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Boids".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                present_mode: bevy::window::PresentMode::Immediate,
                ..default()
            }),
            ..Default::default()
        }))
        .add_systems(Update, (update_boids, separate_boids))
        .add_systems(Startup, startup)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for _ in 0..BOID_COUNT {
        spawn_boid(&mut commands, &mut meshes, &mut materials);
    }
    commands.spawn(Camera2d);
}

#[derive(Component)]
struct Boid {
    direction: f32,
    velocity: f32,
}

fn spawn_boid(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let mut triangle_mesh = Mesh::new(
        PrimitiveTopology::TriangleStrip,
        RenderAssetUsages::default(),
    );

    let vertices = vec![
        Vec3::new(0.0, 2.0, 0.0),
        Vec3::new(-0.8, -0.3, 0.0),
        Vec3::new(0.0, 0.2, 0.0),
        Vec3::new(0.8, -0.3, 0.0),
    ];

    let indices = vec![0, 1, 2, 0, 2, 3];
    let index = Indices::U16(indices);

    triangle_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    triangle_mesh.insert_indices(index);

    let random_position: Vec3 = Vec3 {
        x: rand::random_range(-WINDOW_WIDTH / 2.0..WINDOW_WIDTH / 2.0),
        y: (rand::random_range(-WINDOW_HEIGHT / 2.0..WINDOW_HEIGHT / 2.0)),
        z: 0.0,
    };
    let direction: f32 = rand::random_range(0.0..2.0 * std::f32::consts::PI);
    let velocity: f32 = rand::random_range(0.0..SPEED);
    // Direction is in Radians not Degrees

    commands.spawn((
        Boid {
            direction: direction,
            velocity: velocity,
        },
        Mesh2d(meshes.add(triangle_mesh)),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::WHITE))),
        Transform {
            scale: Vec3 {
                x: 15.0,
                y: 15.0,
                z: 1.0,
            },
            translation: random_position,
            rotation: Quat::from_rotation_z(-direction as f32),
            ..default()
        },
    ));
}

fn distance_equation(x: f64) -> f64 {
    return f64::consts::E.powf(-x / 100.0);
}

fn separate_boids(mut query: Query<(&mut Boid, &mut Transform)>, mut gizmos: Gizmos) {
    let positions: Vec<Vec3> = query
        .iter()
        .map(|(_, transform)| transform.translation)
        .collect();

    for (i, (mut boid, transform)) in query.iter_mut().enumerate() {
        let mut count = 0.0;
        let mut relative_pos = transform.translation.clone();

        for (j, &other_pos) in positions.iter().enumerate() {
            if i == j {
                continue;
            }

            let distance = transform.translation.distance(other_pos);
            if distance < BOID_RADIUS && distance > 0.0 {
                relative_pos += (transform.translation - other_pos).normalize() / distance;
                gizmos.line_2d(
                    transform.translation.xy(),
                    other_pos.xy(),
                    Srgba::rgba_u8(
                        0,
                        255,
                        0,
                        (255.0 * distance_equation(distance as f64)) as u8,
                    ),
                );
                count += 1.0;
            }
        }
    }
}

fn update_boids(
    mut query: Query<(Entity, &mut Boid, &mut Transform)>,
    time: Res<Time>,
    mut gizmos: Gizmos,
    mut counter: Local<u16>,
) {
    for (_entity, mut boid, mut transform) in query.iter_mut() {
        let delta_time: f32 = time.delta_secs();
        // Basic physics
        transform.translation.x += boid.direction.sin() as f32 * boid.velocity * delta_time;
        transform.translation.y += boid.direction.cos() as f32 * boid.velocity * delta_time;

        if *counter < 500 {
            *counter += 1;
        } else {
            *counter = 0;
            boid.direction += rand::random_range(-0.25..0.25);
        }
        //Move the Boid if it wanders off the screen
        if transform.translation.x.abs() > (WINDOW_WIDTH / 2.0) {
            transform.translation.x = -transform.translation.x.signum() * (WINDOW_WIDTH / 2.0);
        }
        if transform.translation.y.abs() > (WINDOW_HEIGHT / 2.0) {
            transform.translation.y = -transform.translation.y.signum() * (WINDOW_HEIGHT / 2.0);
        }

        gizmos.circle_2d(
            Isometry2d::from_translation(transform.translation.xy()),
            BOID_RADIUS,
            LinearRgba::RED,
        );

        transform.rotation = Quat::from_rotation_z(-boid.direction as f32)
    }
}
