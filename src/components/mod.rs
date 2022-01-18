use bevy::prelude::*;

#[derive(Component)]
pub struct LifeUI;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Life {
    pub current: f32,
    pub max: f32,
}
impl Life {
    pub fn with_max(max: f32) -> Self {
        Life {
            current: max,
            max,
        }
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
    North, South, East, West
}
