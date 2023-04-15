use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution},
};
const BG_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const NUMBER_OF_ENEMIES: u32 = 10;

mod player_systems;
mod enemys_systems;
mod bullets_systems;
use player_systems::*;
use enemys_systems::*;
use bullets_systems::*;

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
        .add_startup_system(player_systems::spawn_player)
        .add_startup_system(player_systems::spawn_camera)
        .add_system(player_systems::player_movement)
        .add_startup_system(enemys_systems::spawn_enemy)
        .add_system(enemys_systems::enemy_movement)
        .add_system(bullets_systems::bullet_spawner)
        .add_system(bullets_systems::bullet_movement)
        .run();
}