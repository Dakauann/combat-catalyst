use bevy::{prelude::*, window::PrimaryWindow, sprite::Anchor};

#[derive(Component)]
pub struct Player {}

pub fn spawn_player(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
    let window = window.get_single().unwrap();

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
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..Default::default()
        },
        Player {},
    ));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut Direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::W) {
            Direction.y += 1.0;
        }

        if keyboard_input.pressed(KeyCode::S) {
            Direction.y -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::A) {
            Direction.x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::D) {
            Direction.x += 1.0;
        }

        if Direction.length() > 0.0 {
            Direction = Direction.normalize();
        }

        if transform.translation.x > 1280.0 {
            transform.translation.x = 1280.0;
        }

        if transform.translation.y > 720.0 {
            transform.translation.y = 720.0;
        }


        transform.translation += Direction * 5.0;
    }
}

pub fn spawn_camera(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
    let window = window.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..Default::default()
    });
}