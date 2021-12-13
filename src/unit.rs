use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::cannon::*;
use crate::movement::*;

pub fn spawn_player(mut commands: Commands) {
    let fill = Color::rgb(0.8, 0.1, 0.1);
    let stroke = Color::rgb(0.4, 0.05, 0.05);

    let shape = shapes::Circle {
        radius: 20.0,
        ..shapes::Circle::default()
    };

    commands
        .spawn()
        .insert(Target::default())
        .insert(Player)
        .insert(Movement {
            thrust: 500.0,
            friction: 1.0,
            ..Movement::default()
        })
        .insert(InputDirection::default())
        .insert_bundle(GeometryBuilder::build_as(
            &shape,
            ShapeColors::outlined(fill, stroke),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default()
                    .with_line_width(5.0)
                    .with_line_join(LineJoin::Round),
            },
            Transform::default(),
        ))
        .with_children(|parent| {
            spawn_cannon(parent);
        });
}
