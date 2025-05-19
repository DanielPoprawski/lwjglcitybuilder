use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;
const SPEED: f32 = 0.5;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Boids".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                ..default()
            }),
            ..Default::default()
        }))
        .add_systems(Update, (update_boids, seperate_boids))
        .add_systems(Startup, startup)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for _ in 0..5 {
        spawn_boid(&mut commands, &mut meshes, &mut materials);
    }
    commands.spawn(Camera2d);
}

#[derive(Component)]
struct Boid {
    direction: f32,
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
    // Direction is in Radians not Degrees

    commands.spawn((
        Boid {
            direction: direction,
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

fn seperate_boids(mut query: Query<(&mut Boid, &mut Transform)>) {
    let mut adjacent_boids: Vec<(&mut Boid, &mut Transform)> = Vec::new();
    let mut count: u8 = 0;
    let mut separation_dir: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let iter = query.iter();

    while let Some(a) = iter.next() {
        for b in iter.remaining() {
            if a.1.translation.distance(b.1.translation) < 300.0 {
                separation_dir += a.1.translation - b.1.translation;
                count += 1;
            }
        }
        if count > 0 {
            separation_dir /= Vec3 {
                x: count as f32,
                y: count as f32,
                z: count as f32,
            };
            separation_dir = separation_dir.normalize();
            a.0.direction = separation_dir.x.atan2(separation_dir.y);
        }
    }
}

fn update_boids(mut query: Query<(Entity, &Boid, &mut Transform)>) {
    for (_entity, boid, mut transform) in query.iter_mut() {
        // Basic physics
        transform.translation.x += boid.direction.sin() as f32 * SPEED;
        transform.translation.y += boid.direction.cos() as f32 * SPEED;

        //Move the Boid if it wanders off the screen
        if transform.translation.x.abs() > (WINDOW_WIDTH / 2.0) {
            transform.translation.x = -transform.translation.x.signum() * (WINDOW_WIDTH / 2.0);
        }
        if transform.translation.y.abs() > (WINDOW_HEIGHT / 2.0) {
            transform.translation.y = -transform.translation.y.signum() * (WINDOW_HEIGHT / 2.0);
        }
    }
}
