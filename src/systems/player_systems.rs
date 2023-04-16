use crate::Enemy;
use bevy::{prelude::*, sprite::Anchor, window::PrimaryWindow};
use crate::Player;

pub fn spawn_player(
    mut commands: Commands,
    query: Query<&Transform, With<Player>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window.get_single().unwrap();

    if query.iter().count() == 0 {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    flip_x: false,
                    flip_y: false,
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    anchor: Anchor::Center,
                    ..Default::default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            },
            Player { speed: 3.0 },
        ));

        println!("Player spawned");
    }
}

pub fn player_movement(
    mut player_query: Query<(&mut Transform, &Player)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut transform, player) in player_query.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += player.speed;
        }
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= player.speed;
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= player.speed;
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += player.speed;
        }
    }
}
