use bevy::prelude::*;

use crate::components;

pub fn burn_down_system(
    mut commands: Commands,
    mut burn_query: Query<(Entity, &mut components::Burn, &Parent)>,
    mut damage_query: Query<&mut components::Health>,
    mut burnable_query: Query<&mut components::Burnable>,
    time: Res<Time>,
) {
    for (entity, mut burn, parent) in burn_query.iter_mut() {
        burn.fuel -= time.delta_seconds();
        if burn.fuel <= 0.0 {
            commands.entity(entity).despawn();
        }
        if let Ok(mut health) = damage_query.get_component_mut::<components::Health>(parent.0) {
            health.current -= burn.strength * time.delta_seconds();
        }
        if let Ok(mut burnable) = burnable_query.get_component_mut::<components::Burnable>(entity) {
            burnable.burning = false;
        }
    }
}
