use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use heron::prelude::*;

use crate::components;
use crate::events;
use crate::resources;

pub fn player_system(
    mut player_shape_query: Query<&mut Path, With<components::Player>>,
    mut player_velocity_query: Query<(&mut Velocity, &mut components::Direction), With<components::Player>>,
    mut movements: EventReader<events::InputEvent>,
    shapes: Res<resources::Shapes>,
) {
    let mut player_shape = player_shape_query.single_mut();
    let (mut player_velocity, mut direction) = player_velocity_query.single_mut();
    for event in movements.iter() {
        match *event {
            events::InputEvent::MoveNorth => {
                *player_shape = ShapePath::build_as(&shapes.object_north);
                player_velocity.linear = Vec3::new(0.0, 10.0, 0.0);
                *direction = components::Direction::North;
            },
            events::InputEvent::MoveSouth => {
                *player_shape = ShapePath::build_as(&shapes.object_south);
                player_velocity.linear = Vec3::new(0.0, -10.0, 0.0);
                *direction = components::Direction::South;
            },
            events::InputEvent::MoveEast => {
                *player_shape = ShapePath::build_as(&shapes.object_east);
                player_velocity.linear = Vec3::new(10.0, 0.0, 0.0);
                *direction = components::Direction::East;
            },
            events::InputEvent::MoveWest => {
                *player_shape = ShapePath::build_as(&shapes.object_west);
                player_velocity.linear = Vec3::new(-10.0, 0.0, 0.0);
                *direction = components::Direction::West;
            },
            events::InputEvent::Stop => {
                player_velocity.linear = Vec3::new(0.0, 0.0, 0.0);
            },
        }
    }
}
