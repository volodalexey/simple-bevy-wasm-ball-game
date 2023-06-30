use bevy::{
    prelude::{
        default, Camera3dBundle, Commands, EventReader, OrthographicProjection, Projection, Query,
        Transform, Vec3, With,
    },
    window::{PrimaryWindow, Window, WindowResized},
};

use crate::systems::CAMERA_FAR;

use super::components::MyGameCamera;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        Camera3dBundle {
            projection: Projection::Orthographic(OrthographicProjection {
                far: CAMERA_FAR + 100.0,
                ..default()
            }),
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, CAMERA_FAR)
                .looking_at(
                    Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0),
                    Vec3::Y,
                ),
            ..default()
        },
        MyGameCamera,
    ));
}

pub fn on_resize_window(
    mut resize_reader: EventReader<WindowResized>,
    mut camera_query: Query<&mut Transform, With<MyGameCamera>>,
) {
    let mut camera_transform = camera_query.single_mut();
    for e in resize_reader.iter() {
        camera_transform.translation.x = e.width / 2.0;
        camera_transform.translation.y = e.height / 2.0;
        // When resolution is being changed
        println!("{:.1} x {:.1}", e.width, e.height);
    }
}
