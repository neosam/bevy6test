use bevy::prelude::*;

use crate::components;

pub fn shape_update_system(
    mut query: Query<
        (
            &mut TextureAtlasSprite,
            &components::Direction,
            &components::CreatureShapes,
        ),
        Changed<components::Direction>,
    >,
) {
    for (mut sprite, direction, shapes) in query.iter_mut() {
        match *direction {
            components::Direction::North => sprite.index = shapes.north,
            components::Direction::South => sprite.index = shapes.south,
            components::Direction::East => sprite.index = shapes.east,
            components::Direction::West => sprite.index = shapes.west,
        };
    }
}
