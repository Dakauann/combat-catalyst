use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution},
};

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
    pub size: Vec2,
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}
#[derive(Resource)]
pub struct BulletShootingTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        }
    }
}

impl Default for BulletShootingTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.05, TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub current_state: PlayerState,
    pub set_state: PlayerState,
}

#[derive(PartialEq)]
pub enum PlayerState {
    Idle,
    Running,
}

#[derive(Component)]
pub struct Bullet {
    pub speed: f32,
    pub direction: Vec2,
}

// animation structs
#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);