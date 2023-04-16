use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution},
};
mod structs;
mod systems;

use structs::*;
use systems::bullet_systems;
use systems::camera_systems;
use systems::enemy_systems;
use systems::player_systems;

fn main() {
    App::new()
        .init_resource::<EnemySpawnTimer>()
        .init_resource::<BulletShootingTimer>()
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
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_systems((
            player_systems::spawn_player,
            camera_systems::spawn_camera,
        ))
        .add_systems((
            player_systems::player_movement,
            enemy_systems::spawn_enemys,
            enemy_systems::enemy_movement,
            enemy_systems::tick_enemy_spawn_timer,
            camera_systems::camera_movement,
            bullet_systems::spawn_bullet,
            bullet_systems::bullet_movement,
            bullet_systems::bullet_hit_detection,
            bullet_systems::tick_bullet_shooting_timer,
        ))
        .run();
}
