use bevy::{prelude::*, asset::LoadState};

use crate::{resources, state};

pub fn loading_startup(
    mut handles: ResMut<resources::GraphicsHandles>,
    asset_server: Res<AssetServer>,
) {
    handles.handles = asset_server.load_folder("graphics").expect("Could not load graphics folder");
}

pub fn loading_update(
    handles: Res<resources::GraphicsHandles>,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<state::State>>,
) {
    if asset_server.get_group_load_state(handles.handles.iter().map(|handle| handle.id)) == LoadState::Loaded {
        let handle_count = handles.handles.len();
        bevy::log::info!("Loaded {handle_count} textures");
        state.set(state::State::PostLoading).expect("Could not change state to PostLoading");
    }
}

pub fn post_loading_startup(
    mut commands: Commands,
    handles: Res<resources::GraphicsHandles>,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Image>>,
    mut state: ResMut<State<state::State>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in handles.handles.iter() {
        let texture = textures.get(handle).expect("Texture asset not found");
        texture_atlas_builder.add_texture(handle.clone_weak().typed::<Image>(), texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures)
        .expect("Could not create texture atlas");
    let sprite_indices = resources::SpriteIndices {
        object_north: texture_atlas.get_texture_index(&asset_server.get_handle("graphics/north.png")).unwrap(),
        object_south: texture_atlas.get_texture_index(&asset_server.get_handle("graphics/south.png")).unwrap(),
        object_east: texture_atlas.get_texture_index(&asset_server.get_handle("graphics/east.png")).unwrap(),
        object_west: texture_atlas.get_texture_index(&asset_server.get_handle("graphics/west.png")).unwrap(),
        tree: texture_atlas.get_texture_index(&asset_server.get_handle("graphics/tree.png")).unwrap(),
        camp: texture_atlas.get_texture_index(&asset_server.get_handle("graphics/camp.png")).unwrap(),
        fire: texture_atlas.get_texture_index(&asset_server.get_handle("graphics/fire.png")).unwrap(),
        bullet: texture_atlas.get_texture_index(&asset_server.get_handle("graphics/bullet.png")).unwrap(),

        atlas_handle: texture_atlases.add(texture_atlas),
    };

    commands.insert_resource(sprite_indices);


    state.set(state::State::Ingame).expect("Could not switch state to Ingame");
}
