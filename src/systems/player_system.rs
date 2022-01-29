use bevy::prelude::*;
use heron::prelude::*;

use crate::components;
use crate::events;

pub fn player_system(
    mut player_velocity_query: Query<
        (&mut Velocity, &mut components::Direction),
        With<components::Player>,
    >,
    mut movements: EventReader<events::InputEvent>,
) {
    let (mut player_velocity, mut direction) = player_velocity_query.single_mut();
    for event in movements.iter() {
        match *event {
            events::InputEvent::MoveNorth => {
                player_velocity.linear = Vec3::new(0.0, 2.0, 0.0);
                *direction = components::Direction::North;
            }
            events::InputEvent::MoveSouth => {
                player_velocity.linear = Vec3::new(0.0, -2.0, 0.0);
                *direction = components::Direction::South;
            }
            events::InputEvent::MoveEast => {
                player_velocity.linear = Vec3::new(2.0, 0.0, 0.0);
                *direction = components::Direction::East;
            }
            events::InputEvent::MoveWest => {
                player_velocity.linear = Vec3::new(-2.0, 0.0, 0.0);
                *direction = components::Direction::West;
            }
            events::InputEvent::Stop => {
                player_velocity.linear = Vec3::new(0.0, 0.0, 0.0);
            }
            _ => (),
        }
    }
}
