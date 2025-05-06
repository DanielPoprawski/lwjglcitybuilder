use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins)
      .add_systems(Update, update)
      .add_systems(Startup, startup)
        .run();
}

fn update() {
      
}

fn startup() {
      print!("Hello world!")
}
