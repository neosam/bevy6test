use bevy::prelude::*;
use heron::prelude::*;

use crate::components;

pub fn physics_handler(
    mut query: Query<&mut Velocity, With<components::Movable>>,
) {
    for mut velocity in query.iter_mut() {
        velocity.linear = Vec3::ZERO;
    } 
}
