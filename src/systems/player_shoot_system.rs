use bevy::prelude::*;

use crate::{bundle, components, events, resources};

pub fn player_shoot_system(
    mut commands: Commands,
    query: Query<(&components::Direction, &GlobalTransform), With<components::Player>>,
    mut movements: EventReader<events::InputEvent>,
    shapes: Res<resources::Shapes>,
) {
    for input in movements.iter() {
        match *input {
            events::InputEvent::Shoot => {
                let (direction, player_transform) = query.single();
                let shoot_spawn = match *direction {
                    components::Direction::North => Vec3::new(
                        player_transform.translation.x,
                        player_transform.translation.y + 1.1,
                        player_transform.translation.z,
                    ),
                    components::Direction::South => Vec3::new(
                        player_transform.translation.x,
                        player_transform.translation.y - 1.1,
                        player_transform.translation.z,
                    ),
                    components::Direction::East => Vec3::new(
                        player_transform.translation.x + 1.1,
                        player_transform.translation.y,
                        player_transform.translation.z,
                    ),
                    components::Direction::West => Vec3::new(
                        player_transform.translation.x - 1.1,
                        player_transform.translation.y,
                        player_transform.translation.z,
                    ),
                };
                commands.spawn_bundle(bundle::BulletBundle::new(
                    &shapes,
                    direction,
                    shoot_spawn,
                    0.1,
                    4.0,
                ));
            }
            _ => (),
        }
    }
}