use crate::{structs::BulletShootingTimer, Bullet, Enemy, Player};
use bevy::{prelude::*, sprite::Anchor, window::PrimaryWindow};
use std::thread;

pub fn spawn_bullet(
    mut commands: Commands,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Bullet>)>,
    bullet_shooting_timer: Res<BulletShootingTimer>,
) {
    if bullet_shooting_timer.timer.finished() {
        if enemy_query.iter().count() > 0 {
            let player_transform = player_query.get_single().unwrap();

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
                        flip_x: false,
                        flip_y: false,
                        custom_size: Some(Vec2::new(5.0, 5.0)),
                        anchor: Anchor::Center,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        player_transform.translation.x,
                        player_transform.translation.y,
                        0.0,
                    ),
                    ..Default::default()
                },
                Bullet {
                    speed: 5.0,
                    direction: Vec2::new(0.0, 0.0),
                },
            ));
        }
    }
}

pub fn tick_bullet_shooting_timer(
    mut bullet_shooting_timer: ResMut<BulletShootingTimer>,
    time: Res<Time>,
) {
    bullet_shooting_timer.timer.tick(time.delta());
}

pub fn bullet_movement(
    mut bullet_query: Query<(&mut Transform, &mut Bullet)>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Bullet>)>,
) {
    for (mut transform, mut bullet) in bullet_query.iter_mut() {
        let mut neared_enemy_direction;

        if enemy_query.iter().count() == 0 {
            neared_enemy_direction = Vec3::new(
                rand::random::<f32>() * 100.0,
                rand::random::<f32>() * 100.0,
                0.0,
            );
        } else {
            neared_enemy_direction = enemy_query
                .iter()
                .map(|enemy_transform| {
                    (
                        enemy_transform.translation,
                        (enemy_transform.translation - transform.translation).length(),
                    )
                })
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap()
                .0;
        }

        if bullet.direction == Vec2::new(0.0, 0.0) {
            bullet.direction = Vec2::new(
                neared_enemy_direction.x - transform.translation.x,
                neared_enemy_direction.y - transform.translation.y,
            )
            .normalize()
                * bullet.speed;

            transform.translation.x += bullet.direction.x;
            transform.translation.y += bullet.direction.y;
        } else {
            transform.translation.x += bullet.direction.x;
            transform.translation.y += bullet.direction.y;
        }
    }
}

pub fn bullet_hit_detection(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &Transform, &Bullet)>,
    mut enemy_query: Query<(Entity, &Transform, &Enemy)>,
) {
    for (bullet_entity, bullet_transform, bullet) in bullet_query.iter_mut() {
        if enemy_query.iter().count() == 0 {
            return;
        }

        for (enemy_entity, enemy_transform, enemy) in enemy_query.iter_mut() {
            let x_distance = (enemy_transform.translation.x - bullet_transform.translation.x).abs();
            let y_distance = (enemy_transform.translation.y - bullet_transform.translation.y).abs();

            // Check if the bullet collides with the enemy
            if x_distance < enemy.size.x / 2.0 && y_distance < enemy.size.y / 2.0 {
                // Remove the bullet and enemy entities
                commands.entity(bullet_entity).despawn();
                commands.entity(enemy_entity).despawn();
            }
        }
    }
}
