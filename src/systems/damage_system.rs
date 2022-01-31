use bevy::{prelude::*, utils::HashSet};

use crate::{events, components};

pub fn damage_system(
    mut commands: Commands,
    mut damager_health_events: EventReader<events::DamagerHealthEvent>,
    mut damager_health_pairs: Local<HashSet<(Entity, Entity)>>,
    mut damager_query: Query<&mut components::Damager>,
    mut health_query: Query<&mut components::Health>,
    time: Res<Time>,
) {
    for damager_health_event in damager_health_events.iter() {
        let pair = (damager_health_event.damager_entity, damager_health_event.health_entity);
        match damager_health_event.stage {
            events::EventStage::Started => damager_health_pairs.insert(pair),
            events::EventStage::Stopped => damager_health_pairs.remove(&pair),
        };

        for (damager_entity, health_entity) in damager_health_pairs.iter() {
            if let (Ok(damager), Ok(mut health)) = (
                damager_query.get_component_mut::<components::Damager>(*damager_entity),
                health_query.get_component_mut::<components::Health>(*health_entity)
            ) {
                let strength = damager.strength;
                if damager.single_hit {
                    health.current -= strength;
                    commands.entity(*damager_entity).despawn_recursive();
                } else {
                    health.current -= strength * time.delta_seconds();
                }
            }
        }
    }
}