use bevy::prelude::*;

pub struct Shapes {
    pub object_north: Handle<Image>,
    pub object_south: Handle<Image>,
    pub object_east: Handle<Image>,
    pub object_west: Handle<Image>,
}
impl Default for Shapes {
    fn default() -> Self {
        Shapes {
            object_north: Handle::default(),
            object_south: Handle::default(),
            object_east: Handle::default(),
            object_west: Handle::default(),
        }
    }
}

#[derive(Default)]
pub struct InputStore {
    pub player_move_north: bool,
    pub player_move_south: bool,
    pub player_move_east: bool,
    pub player_move_west: bool,
}