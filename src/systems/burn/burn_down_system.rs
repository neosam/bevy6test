use bevy::prelude::*;

use crate::components;

pub fn burn_down_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut components::Burn)>,
    time: Res<Time>,
) {
    for (entity, mut burn) in query.iter_mut() {
        burn.fuel -= time.delta_seconds();
        if burn.fuel <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}