use std::f32::consts::{FRAC_2_PI, PI};

#[warn(unused_imports)]
use bevy::{
    asset::RenderAssetUsages,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

use noise::{NoiseFn, Perlin};

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;
const SPEED: f32 = 300.;
const BOID_COUNT: u8 = 15;
const BOID_RADIUS: f32 = 150.0;

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
        .add_systems(Update, (update_boids, separation, cohesion, alignment))
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
    noise: Perlin,
    noise_offset: f32,
}

impl Boid {
    fn get_velocity(&self) -> Vec2 {
        Vec2 {
            x: self.direction.cos() * self.velocity,
            y: self.direction.sin() * self.velocity,
        }
    }
}

fn is_in_range(pos: Vec3, direction: f32, other_pos: Vec3) -> bool {
    if pos.distance(other_pos) > BOID_RADIUS {
        return false;
    }

    let angle_to_other = (other_pos.y - pos.y).atan2(other_pos.x - pos.x);

    // Convert your direction to atan2's coordinate system
    let direction_in_atan2_coords = direction - PI / 2.0;

    // Now calculate behind angle
    let behind_angle = (direction_in_atan2_coords + PI) % (2.0 * PI);

    let angle_diff = (angle_to_other - behind_angle).abs();
    let angle_diff = if angle_diff > PI {
        2.0 * PI - angle_diff
    } else {
        angle_diff
    };

    let exclusion_zone = PI / 3.0; // 60 degrees
    angle_diff > exclusion_zone / 2.0
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

    let direction = rand::random_range(0.0..2.0 * std::f32::consts::PI); // Direction is in Radians not Degrees

    commands.spawn((
        Boid {
            direction: direction,
            velocity: SPEED,
            noise: Perlin::new(rand::random()),
            noise_offset: rand::random::<f32>() * 1000.0,
        },
        Mesh2d(meshes.add(triangle_mesh)),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::WHITE))),
        Transform {
            scale: Vec3 {
                x: 15.0,
                y: 15.0,
                z: 1.0,
            },
            translation: Vec3 {
                x: rand::random_range(-WINDOW_WIDTH / 2.0..WINDOW_WIDTH / 2.0),
                y: (rand::random_range(-WINDOW_HEIGHT / 2.0..WINDOW_HEIGHT / 2.0)),
                z: 0.0,
            },
            rotation: Quat::from_rotation_z(direction),
            ..default()
        },
    ));
}

fn update_boids(
    mut query: Query<(Entity, &mut Boid, &mut Transform)>,
    time: Res<Time>,
    // mut gizmos: Gizmos,
) {
    for (_entity, mut boid, mut transform) in query.iter_mut() {
        if boid.direction > 2.0 * PI {
            boid.direction -= 2.0 * PI; // Make sure direction is always between 0 and 2 PI
        }
        let delta_time: f32 = time.delta_secs();
        // Basic physics
        transform.translation.x += boid.direction.cos() as f32 * boid.velocity * delta_time;
        transform.translation.y += boid.direction.sin() as f32 * boid.velocity * delta_time;

        //Move the Boid if it wanders off the screen
        if transform.translation.x.abs() > (WINDOW_WIDTH / 2.0) {
            transform.translation.x = -transform.translation.x.signum() * (WINDOW_WIDTH / 2.0);
        }
        if transform.translation.y.abs() > (WINDOW_HEIGHT / 2.0) {
            transform.translation.y = -transform.translation.y.signum() * (WINDOW_HEIGHT / 2.0);
        }

        //Random wandering
        boid.noise_offset += delta_time;
        let noise_value = boid.noise.get([boid.noise_offset as f64; 2]) as f32;
        boid.direction += noise_value * 0.001;

        // gizmos.circle_2d(
        //     Isometry2d::from_translation(transform.translation.xy()),
        //     BOID_RADIUS,
        //     LinearRgba::RED,
        // );

        transform.rotation = Quat::from_rotation_z(boid.direction - PI / 2.0);
    }
}

fn separation(mut query: Query<(Entity, &mut Boid, &mut Transform)>, mut gizmos: Gizmos) {
    for [(e_a, b_a, t_a), (e_b, b_b, t_b)] in query.iter_combinations() {
        if e_a == e_b {
            continue;
        }

        if !is_in_range(t_a.translation, b_a.direction, t_b.translation) {
            continue;
        }

        let relative_position: Vec2 = (t_a.translation - t_b.translation).xy();
        let relative_velocity: Vec2 = b_a.get_velocity() - b_b.get_velocity();

        let dot_product = relative_velocity.dot(relative_position);
        // if dot_product < 0. {
        //     gizmos.line_2d(
        //         t_a.translation.xy(),
        //         t_a.translation.xy() + b_a.get_velocity().normalize() * 150.0,
        //         Srgba::rgb_u8(255, 0, 0),
        //     );
        //     gizmos.line_2d(
        //         t_b.translation.xy(),
        //         t_b.translation.xy() + b_b.get_velocity().normalize() * 150.0,
        //         Srgba::rgb_u8(0, 255, 0),
        //     );
        // }
    }
}

fn alignment(
    mut query: Query<(Entity, &mut Boid, &mut Transform)>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let boid_data: Vec<(Entity, f32, Vec3)> = query
        .iter()
        .map(|(entity, boid, transform)| (entity, boid.direction, transform.translation))
        .collect();

    for (entity, mut boid, transform) in query.iter_mut() {
        for &(other_entity, other_boid, other_transform) in &boid_data {
            if entity == other_entity {
                continue;
            }

            if !is_in_range(transform.translation, boid.direction, other_transform) {
                commands.entity(entity).insert(MeshMaterial2d(
                    materials.add(ColorMaterial::from_color(Color::WHITE)),
                ));
                continue;
            }
            commands.entity(entity).insert(MeshMaterial2d(
                materials.add(ColorMaterial::from_color(Color::srgb_u8(255, 0, 0))),
            ));

            let direction_difference = (boid.direction - other_boid).abs();
            if boid.direction > other_boid {
                boid.direction -= direction_difference * 0.00125;
            } else {
                boid.direction += direction_difference * 0.00125;
            }
        }
    }
}
fn cohesion(mut query: Query<(Entity, &mut Boid, &mut Transform)>, mut gizmos: Gizmos) {
    let boid_data: Vec<(Entity, f32, Vec3)> = query
        .iter()
        .map(|(entity, boid, transform)| (entity, boid.direction, transform.translation))
        .collect();

    for (entity, mut boid, transform) in query.iter_mut() {
        for &(other_entity, other_boid, other_transform) in &boid_data {
            if entity == other_entity {
                continue;
            }

            let distance = transform.translation.distance(other_transform);
            if distance > BOID_RADIUS {
                continue;
            }
        }
    }
}
