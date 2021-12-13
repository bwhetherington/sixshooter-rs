use bevy::{math::{Vec3Swizzles, Vec4Swizzles}, prelude::*};
use bevy_prototype_lyon::prelude::*;

use crate::MainCamera;

#[derive(Default)]
pub struct Cannon {
    pub target: Vec2,
}

pub struct CannonPlugin;

fn update_cannon_transform(time: Res<Time>, mut query: Query<(&Cannon, &mut Transform, &GlobalTransform)>) {
    for (cannon, mut transform, global_transform) in query.iter_mut() {
        transform.translation.x = 10.0;
    }
}

#[derive(Default)]
pub struct Target(pub Vec2);

fn update_target(
    windows: Res<Windows>,
    q_camera: Query<&Transform, With<MainCamera>>,
    mut query: Query<&mut Target>,
) {
    let window = windows.get_primary().unwrap();

    // check if the cursor is in the primary window
    if let Some(pos) = window.cursor_position() {
        // get the size of the window
        let size = Vec2::new(window.width() as f32, window.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = pos - size / 2.0;

        // assuming there is exactly one main camera entity, so this is OK
        let camera_transform = q_camera.single().unwrap();

        // apply the camera transform
        let pos_wld = (camera_transform.compute_matrix() * p.extend(0.0).extend(1.0)).xy();
        
        for mut target in query.iter_mut() {
            target.0 = pos_wld;
        }
    }
}

pub fn spawn_cannon(parent: &mut ChildBuilder) {
    let shape = shapes::Rectangle {
        width: 40.0,
        height: 20.0,
        ..shapes::Rectangle::default()
    };
    let fill = Color::rgb(0.8, 0.1, 0.1);
    let stroke = Color::rgb(0.4, 0.05, 0.05);
    parent
        .spawn()
        .insert(Cannon::default())
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
        ));
}

impl Plugin for CannonPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(
                update_cannon_transform
                    .system()
            )
            .add_system(
                update_target.system()
            );
    }
}
