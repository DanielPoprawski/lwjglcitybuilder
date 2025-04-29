use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Flappy Bird"),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: Vec2::new(512., 512.).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(Update, update_bird)
        .add_systems(Startup, setup_level)
        .run();
}

const PIXEL_RATIO: f32 = 4.0;
#[derive(Component)]
struct Bird {
    pub velocity: f32,
}

fn setup_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ClearColor(Color::srgb(0.5, 0.7, 0.8)));
    commands.spawn(Camera2d::default());
    commands.spawn((
        Sprite {
            image: asset_server.load("bird.png"),
            ..Default::default()
        },
        Transform::IDENTITY.with_scale(Vec3::splat(PIXEL_RATIO)),
        Bird { velocity: 0.0 },
    ));
}

const FLAP_FORCE: f32 = 500.;
const GRAVITY: f32 = 2000.;
const VELOCITY_TO_ROTATION_RATIO: f32 = 7.5;

const OBSTACLE_AMOUNT: i32 = 5;
const OBSTACLE_WIDTH: f32 = 32.;
const OBSTACLE_HEIGHT: f32 = 144.;
const OBSTACLE_VERTICAL_OFFSET: f32 = 30.;
const OBSTACLE_GAP_SIZE: f32 = 15.;
const OBSTACLE_SPACING: f32 = 60.;
const OBSTACLE_SCROLL_SPEED: f32 = 150.;

fn update_bird(
    mut bird_query: Query<(&mut Bird, &mut Transform)>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if let Ok((mut bird, mut transform)) = bird_query.single_mut() {
        if keys.just_pressed(KeyCode::Space) {
            bird.velocity = FLAP_FORCE;
        }

        bird.velocity -= time.delta_secs() * GRAVITY;
        transform.translation.y += bird.velocity * time.delta_secs();

        transform.rotation = Quat::from_axis_angle(
            Vec3::Z,
            f32::clamp(bird.velocity / VELOCITY_TO_ROTATION_RATIO, -90., 90.).to_radians(),
        );
    }
}
