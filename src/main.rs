use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution},
};
mod lib;
mod systems;

use lib::enemy_structs;
use systems::camera_systems;
use systems::enemy_systems;
use systems::player_systems;

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
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(enemy_structs::EnemysInfos { enemys_count: 30 })
        .add_startup_systems((
            player_systems::spawn_player,
            camera_systems::spawn_camera,
            enemy_systems::spawn_enemys,
        ))
        .add_systems((
            player_systems::player_movement,
            camera_systems::camera_movement,
        ))
        .run();
}
