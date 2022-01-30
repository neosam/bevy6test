use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Inspectable)]
pub struct LifeUI;

#[derive(Component, Inspectable)]
pub struct Player;

#[derive(Component, Inspectable)]
pub struct MainCamera;

#[derive(Component, Inspectable)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}
impl Health {
    pub fn with_max(max: f32) -> Self {
        Health { current: max, max }
    }
}

#[derive(Component, Inspectable)]
pub struct CreatureShapes {
    pub north: usize,
    pub south: usize,
    pub east: usize,
    pub west: usize,
}

#[derive(Component, Inspectable)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Component, Clone, Inspectable)]
pub struct Burn {
    pub fuel: f32,
    pub radius: f32,
    pub strength: f32,
}

#[derive(Component, Inspectable)]
pub struct Burnable {
    pub resist: f32,
    pub max_resistence: f32,
    pub min_resistence_to_burn: f32,
    pub recover: f32,
    pub inactive: bool,
    pub burning: bool,
    pub burn: Burn,
}

#[derive(Component, Inspectable)]
pub struct Destroyed;

#[derive(Component, Inspectable)]
pub struct AutoDespawn {
    pub time_left: f32,
    pub frames_left: i32,
}
