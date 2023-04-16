use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
}

#[derive(Resource)]
pub struct EnemysInfos {
    pub enemys_count: usize,
}