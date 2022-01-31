use bevy::prelude::*;

use crate::components;

pub fn damage_system(
    mut commands: Commands,
    mut damage_query: Query<
        (Entity, &components::Health, Option<&mut Visibility>),
        (Changed<components::Health>, Without<components::Destroyed>),
    >,
) {
    for (entity, health, visibility) in damage_query.iter_mut() {
        if health.current <= 0.0 {
            commands
                .entity(entity)
                .insert(components::Destroyed)
                .insert(components::AutoDespawn {
                    time_left: 0.0,
                    frames_left: 5,
                });
            if let Some(mut visibility) = visibility {
                visibility.is_visible = false;
            }
        }
    }
}
