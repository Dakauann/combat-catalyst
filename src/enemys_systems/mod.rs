use bevy::{prelude::*, sprite::Anchor, window::PrimaryWindow};
use rand::Rng;

use crate::{player_systems::Player, NUMBER_OF_ENEMIES};

#[derive(Component)]
pub struct Enemy {}

pub fn enemy_movement(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Transform, &Enemy), Without<Player>>,
    time: Res<Time>,
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        if let Ok(player_transform) = player_query.get_single() {
            let player_pos = player_transform.translation;

            let direction = player_pos - transform.translation;

            let speed = 100.0;
            let movement = direction.normalize() * speed * time.delta_seconds();

            transform.translation += movement;
        }
    }
}

pub fn spawn_enemy(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
    let window = window.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x_range = rand::thread_rng().gen_range(0.0..window.width());
        let random_y_range = rand::thread_rng().gen_range(0.0..window.height());

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    flip_x: false,
                    flip_y: false,
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    anchor: Anchor::Center,
                    ..Default::default()
                },
                transform: Transform::from_xyz(random_x_range, random_y_range, 0.0),
                ..Default::default()
            },
            Enemy {},
        ));
    }
}
