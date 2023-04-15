use bevy::{prelude::*, sprite::Anchor};

use crate::{player_systems::Player, enemys_systems::Enemy};

#[derive(Component)]
pub struct Bullet {
    velocity: Vec3,
}

pub fn bullet_spawner(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        if let Some(player_transform) = query.iter().next() {
            let bullet_position = player_transform.translation;
            if enemy_query.iter().next().is_none() {
                return;
            }
            let nearest_enemy = enemy_query
                .iter()
                .min_by(|a, b| {
                    a.translation
                        .distance_squared(bullet_position)
                        .partial_cmp(&b.translation.distance_squared(bullet_position))
                        .unwrap()
                })
                .unwrap();

            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    flip_x: false,
                    flip_y: false,
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    anchor: Anchor::Center,
                    ..Default::default()
                },
                transform: Transform::from_translation(bullet_position),
                ..Default::default()
            })
            .insert(Bullet {
                velocity: (nearest_enemy.translation - bullet_position)
                .normalize()
                    * 500.0,
            });
        }
    }
}

pub fn bullet_movement(
    time: Res<Time>,
    mut bullet_query: Query<(Entity, &mut Transform, &Bullet), Without<Enemy>>,
    mut enemy_query: Query<(Entity, &Transform, &Enemy), Without<Bullet>>,
    mut commands: Commands,
) {
    for (bullet_entity, mut bullet_transform, bullet) in bullet_query.iter_mut() {
        bullet_transform.translation += bullet.velocity * time.delta_seconds();

        for (enemy_entity, enemy_transform, _) in enemy_query.iter_mut() {
            let distance = bullet_transform.translation.distance(enemy_transform.translation);
            if distance < 10.0 {
                commands.entity(bullet_entity).despawn();
                commands.entity(enemy_entity).despawn();
            }
        }

        if bullet_transform.translation.x < 0.0
            || bullet_transform.translation.x > 1280.0
            || bullet_transform.translation.y < 0.0
            || bullet_transform.translation.y > 720.0
        {
            commands.entity(bullet_entity).despawn();
        }
    }
}