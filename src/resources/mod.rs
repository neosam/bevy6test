use bevy::prelude::*;

#[derive(Default)]
pub struct Shapes {
    pub object_north: Handle<Image>,
    pub object_south: Handle<Image>,
    pub object_east: Handle<Image>,
    pub object_west: Handle<Image>,
    pub tree: Handle<Image>,
    pub camp: Handle<Image>,
    pub fire: Handle<Image>,
    pub bullet: Handle<Image>,
}

#[derive(Default)]
pub struct GraphicsHandles {
    pub handles: Vec<HandleUntyped>,
}

#[derive(Default)]
pub struct SpriteIndices {
    pub atlas_handle: Handle<TextureAtlas>,
    pub object_north: usize,
    pub object_south: usize,
    pub object_east: usize,
    pub object_west: usize,
    pub tree: usize,
    pub camp: usize,
    pub fire: usize,
    pub bullet: usize,
}


#[derive(Default)]
pub struct InputStore {
    pub player_move_north: bool,
    pub player_move_south: bool,
    pub player_move_east: bool,
    pub player_move_west: bool,
}
