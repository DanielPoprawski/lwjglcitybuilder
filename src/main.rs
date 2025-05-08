use bevy::prelude::*;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

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
        .add_systems(Update, update)
        .add_systems(Startup, startup)
        .run();
}

fn update() {}

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

fn spawn_boid(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let random_position: Vec3 = Vec3 {
        x: rand::random_range(-WINDOW_WIDTH / 2.0..WINDOW_WIDTH / 2.0),
        y: (rand::random_range(-WINDOW_HEIGHT / 2.0..WINDOW_HEIGHT / 2.0)),
        z: 0.0,
    };
    commands.spawn((
        Mesh2d(meshes.add(Triangle2d::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(-1.0, -2.0),
            Vec2::new(1.0, -2.0),
        ))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::WHITE))),
        Transform {
            scale: Vec3 {
                x: 5.0,
                y: 5.0,
                z: 1.0,
            },
            translation: random_position,
            ..default()
        },
    ));
}
