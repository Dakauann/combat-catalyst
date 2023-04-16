use bevy::{prelude::*, window::PrimaryWindow};
use crate::{Player, Enemy};

pub fn spawn_camera(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
    let window = window.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}

pub fn camera_movement(
    mut camera_query: Query<(&mut Transform, (&Camera, Without<Player>))>,
    player: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
   if let Ok(mut camera_transform) = camera_query.get_single_mut() {
       if let Ok(player_transform) = player.get_single() {
           camera_transform.0.translation.x = player_transform.translation.x;
           camera_transform.0.translation.y = player_transform.translation.y;
       }
    }
}
