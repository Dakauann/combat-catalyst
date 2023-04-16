use crate::structs::AnimationIndices;
use crate::structs::AnimationTimer;
use crate::Player;
use crate::PlayerState;
use bevy::{prelude::*, sprite::Anchor, window::PrimaryWindow};

pub fn animate_player_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("images/idle.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(28.0, 34.0), 4, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 1, last: 3 };

    if query.iter().count() == 0 {
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices.first),

                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            Player {
                speed: 3.0,
                current_state: PlayerState::Idle,
                set_state: PlayerState::Idle,
            },
        ));

        println!("Player spawned");
    }
}

pub fn player_movement(
    mut commands: Commands,
    mut player_transform: Query<(Entity, &mut Transform, &mut Player)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (player_entity, mut transform, mut player) in player_transform.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += player.speed;
        }
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= player.speed;
            transform.scale.x = -1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= player.speed;
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += player.speed;
            transform.scale.x = 1.0;
        }

    }
}
