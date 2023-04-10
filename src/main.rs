use bevy::{prelude::*, sprite::Anchor, window::PrimaryWindow};
use rand::Rng;
const BG_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const NUMBER_OF_ENEMIES: u32 = 10;

#[derive(Component)]
struct Player {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BG_COLOR))
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_enemy)
        .add_system(player_movement)
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
        
        commands.spawn(SpriteBundle {
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
        });
    }
}

fn spawn_camera(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
    let window = window.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..Default::default()
    });
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

        transform.translation += Direction * 5.0;
    }
}
