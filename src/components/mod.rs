use bevy::prelude::*;

#[derive(Component)]
pub struct LifeUI;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}
impl Health {
    pub fn with_max(max: f32) -> Self {
        Health { current: max, max }
    }
}

#[derive(Component)]
pub struct CreatureShapes {
    pub north: Handle<Image>,
    pub south: Handle<Image>,
    pub east: Handle<Image>,
    pub west: Handle<Image>,
}

#[derive(Component)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Component, Clone)]
pub struct Burn {
    pub fuel: f32,
    pub radius: f32,
    pub strength: f32,
}

#[derive(Component)]
pub struct Burnable {
    pub resist: f32,
    pub max_resistence: f32,
    pub min_resistence_to_burn: f32,
    pub recover: f32,
    pub inactive: bool,
    pub burning: bool,
    pub burn: Burn,
}

#[derive(Component)]
pub struct Destroyed;

#[derive(Component)]
pub struct AutoDespawn {
    pub time_left: f32,
    pub frames_left: i32,
}
