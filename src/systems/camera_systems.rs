use bevy::{prelude::*, window::PrimaryWindow};
#[path = "../lib/player_structs.rs"]
mod player_structs;
use player_structs::Player;

pub fn spawn_camera(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
    let window = window.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..Default::default()
    });
}

pub fn camera_movement(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut player_query: Query<(&mut Transform, With<Player>), Without<Camera>>,
) {
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        if let Ok((player_transform, _)) = player_query.get_single_mut() {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}
