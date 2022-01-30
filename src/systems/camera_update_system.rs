use bevy::prelude::*;

use crate::components;

pub fn camera_update_system(
    mut queries: QuerySet<(
        QueryState<&Transform, (With<components::Player>, Changed<Transform>)>,
        QueryState<&mut Transform, With<components::MainCamera>>,
    )>,
) {
    let player_translation = match queries.q0().get_single() {
        Ok(transform) => transform.translation,
        Err(_) => return,
    };
    if let Ok(mut camera_transform) = queries.q1().get_single_mut() {
        camera_transform.translation.x = player_translation.x;
        camera_transform.translation.y = player_translation.y;
    }
}
