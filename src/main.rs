use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod movement;
use movement::MovementPlugin;

mod unit;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup.system())
        .add_startup_system(unit::spawn_player.system())
        .add_plugin(MovementPlugin)
        .run();
}
