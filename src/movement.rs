use bevy::prelude::*;

pub struct MovementPlugin;

#[derive(Clone, Copy)]
pub struct Player;

pub struct Movement {
    pub direction: Vec2,
}

pub struct Velocity(pub Vec2);

fn handle_input(keys: Res<Input<KeyCode>>, mut query: Query<(&Player, &mut Movement)>) {
    let mut direction = Vec2::default();

    if keys.pressed(KeyCode::W) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::S) {
        direction.y -= 1.0;
    }
    if keys.pressed(KeyCode::A) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::D) {
        direction.x += 1.0;
    }

    direction.normalize_or_zero();

    for (_player, mut movement) in query.iter_mut() {
        movement.direction = direction;
    }
}

fn handle_velocity(mut query: Query<(&Movement, &mut Velocity)>) {
    for (movement, mut vel) in query.iter_mut() {
        vel.0 = movement.direction * 100.0;
    }
}

fn handle_movement(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    let dt = time.delta_seconds_f64() as f32;
    for (velocity, mut transform) in query.iter_mut() {
        let vel: Vec3 = Vec3::new(velocity.0.x, velocity.0.y, 0.0);
        transform.translation += vel * dt;
    }
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(handle_input.system().label("input"))
            .add_system(handle_velocity.system().label("velocity").after("input"))
            .add_system(handle_movement.system().label("movement").after("velocity"));
    }
}
