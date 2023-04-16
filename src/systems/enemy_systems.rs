#[path = "../lib/enemy_structs.rs"]
mod enemy_structs;

use bevy::{prelude::*, sprite::Anchor, window::PrimaryWindow};
use enemy_structs::Enemy;

use crate::lib::player_structs::Player;

pub fn spawn_enemys(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(window) = window.get_single() {
        if let Ok(player_transform) = player_query.get_single() {
            for _ in 0..30 {
                let random_x_on_screen = rand::random::<f32>() * window.width();
                let random_y_on_screen = rand::random::<f32>() * window.height();

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
                        transform: Transform::from_xyz(
                            random_x_on_screen,
                            random_y_on_screen,
                            0.0,
                        ),
                        ..Default::default()
                    },
                    Enemy { speed: 1.0 },
                ));
            }
        }
    }
}
