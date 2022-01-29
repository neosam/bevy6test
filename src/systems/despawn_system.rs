use bevy::prelude::*;

use crate::components;

pub fn despawn_system(
    mut commands: Commands,
    mut despawn_query: Query<(Entity, &mut components::AutoDespawn)>,
    time: Res<Time>,
) {
    for (entity, mut auto_despawn) in despawn_query.iter_mut() {
        auto_despawn.time_left -= time.delta_seconds();
        auto_despawn.frames_left -= 1;

        if auto_despawn.time_left <= 0.0 && auto_despawn.frames_left <= 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
