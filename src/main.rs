use bevy::{
    prelude::*,
    sprite::Anchor,
    window::{PresentMode, PrimaryWindow, WindowResolution},
};
use rand::Rng;
const BG_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const NUMBER_OF_ENEMIES: u32 = 10;

#[derive(Component)]
struct Player {}
#[derive(Component)]
struct Enemy {
    Direction: Vec3,
}
#[derive(Component)]
struct Bullet {
    velocity: Vec3,
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Combat Catalyst".into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                resolution: WindowResolution::new(1280.0, 720.0),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BG_COLOR))
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_enemy)
        .add_system(bullet_spawner)
        .add_system(bullet_movement)
        // .add_system(bullet_collision)
        .add_system(player_movement)
        .add_system(enemy_movement)
        .run();
}

fn spawn_player(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
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

fn spawn_enemy(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
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
            Enemy {
                Direction: Vec3::ZERO,
            },
        ));
    }
}

fn spawn_camera(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
    let window = window.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..Default::default()
    });
}

fn bullet_spawner(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        if let Some(player_transform) = query.iter().next() {
            let bullet_position = player_transform.translation;
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

fn bullet_movement(
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

fn enemy_movement(
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

fn player_movement(
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
