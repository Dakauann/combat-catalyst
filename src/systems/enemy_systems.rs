use crate::Enemy;
use crate::EnemySpawnTimer;
use crate::Player;

use bevy::{prelude::*, sprite::Anchor, window::PrimaryWindow};
use rand::thread_rng;
use rand::Rng;
const ENEMY_SIZE: Vec2 = Vec2::new(10.0, 10.0);

pub fn spawn_enemys(
    mut commands: Commands,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
    asset_server: Res<AssetServer>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    if enemy_spawn_timer.timer.finished() {
        let player_transform = player_query.get_single().unwrap();

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    flip_x: false,
                    flip_y: false,
                    custom_size: Some(ENEMY_SIZE),
                    anchor: Anchor::Center,
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    player_transform.translation.x + thread_rng().gen_range(-1000.0..1000.0),
                    player_transform.translation.y + thread_rng().gen_range(-1000.0..1000.0),
                    0.0,
                ),
                ..Default::default()
            },
            Enemy {
                speed: 1.0,
                size: ENEMY_SIZE,
            },
        ));

        println!("Enemy spawned");
    }
}

pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let player_transform = player_query.get_single().unwrap();
        let transform_translation = transform.translation;

        transform.translation += Vec3::new(
            (player_transform.translation.x - transform_translation.x).signum() * enemy.speed,
            (player_transform.translation.y - transform_translation.y).signum() * enemy.speed,
            0.0,
        );
    }
}
