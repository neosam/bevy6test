use bevy::prelude::*;

#[derive(Default)]
pub struct GraphicsHandles {
    pub handles: Vec<Handle<Image>>,
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
