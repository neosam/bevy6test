use bevy::prelude::*;

use crate::components;

pub fn shape_update_system(
    mut query: Query<(&mut Handle<Image>, &components::Direction, &components::CreatureShapes), Changed<components::Direction>>,
) {
    for (mut image, direction, shapes) in query.iter_mut() {
        match *direction {
            components::Direction::North => *image = shapes.north.clone(),
            components::Direction::South => *image = shapes.south.clone(),
            components::Direction::East => *image = shapes.east.clone(),
            components::Direction::West => *image = shapes.west.clone(),
        };
    }
}