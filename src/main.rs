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
        .add_systems(Update, update_boids)
        .add_systems(Startup, startup)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for _ in 0..50 {
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
                x: 5.0,
                y: 5.0,
                z: 1.0,
            },
            translation: random_position,
            rotation: Quat::from_rotation_z(-direction as f32),
            ..default()
        },
    ));
}

fn update_boids(
    mut boids: Query<(Entity, &mut Boid, &mut Transform)>,
    mut others: Query<(Entity, &mut Boid, &mut Transform)>,
) {
    for (entity, boid, mut transform) in boids.iter_mut() {
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

        for (other_entity, other_boid, other_transform) in others.iter() {
            if entity == other_entity {
                continue;
            }
            if (transform.translation.distance(other_transform.translation)) {}
        }
    }
}
