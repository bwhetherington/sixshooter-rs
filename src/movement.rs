use bevy::{prelude::*, math::Vec3Swizzles};

use crate::cannon::Target;

pub struct MovementPlugin;

#[derive(Clone, Copy)]
pub struct Player;

#[derive(Default)]
pub struct InputDirection(pub Vec2);

#[derive(Default)]
pub struct Movement {
    pub thrust: f32,
    pub friction: f32,
    pub acceleration: Vec2,
    pub velocity: Vec2,
}

fn handle_input(keys: Res<Input<KeyCode>>, mut query: Query<&mut InputDirection, With<Player>>) {
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

    for mut movement in query.iter_mut() {
        movement.0 = direction;
    }
}

fn handle_velocity(time: Res<Time>, mut query: Query<(&InputDirection, &mut Movement)>) {
    for (input, mut movement) in query.iter_mut() {
        movement.acceleration = input.0 * movement.thrust;

        let old_sign = Vec2::new(movement.velocity.x.signum(), movement.velocity.y.signum());

        let friction = movement.friction * time.delta_seconds();
        movement.velocity *= (1.0 - friction).max(0.0);

        let acc = movement.acceleration * time.delta_seconds();
        movement.velocity += acc;

        let new_sign = Vec2::new(movement.velocity.x.signum(), movement.velocity.y.signum());

        if new_sign.x != old_sign.x && new_sign.y != new_sign.y {
            movement.velocity = Vec2::ZERO;
        }
    }
}

fn handle_movement(time: Res<Time>, mut query: Query<(&Target, &Movement, &mut Transform)>) {
    let dt = time.delta_seconds();
    for (target, movement, mut transform) in query.iter_mut() {
        let vel: Vec3 = Vec3::new(movement.velocity.x, movement.velocity.y, 0.0);
        transform.translation += vel * dt;

        let to = target.0;
        let from = transform.translation.xy();

        let diff = to - from;
        let mut angle = diff.y.atan2(diff.x);

        if angle.is_nan() {
            angle = 0.0;
        }

        transform.rotation = Quat::from_rotation_z(angle);
    }
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(handle_input.system().label("input"))
            .add_system(handle_velocity.system().label("velocity").after("input"))
            .add_system(handle_movement.system().label("movement").after("velocity"));
    }
}
