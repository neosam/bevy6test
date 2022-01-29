use bevy::{prelude::*, utils::HashSet};

use crate::{components, events};

pub fn burning_system(
    mut burn_burnable_events: EventReader<events::BurnBurnableEvent>,
    mut burn_burnable_pairs: Local<HashSet<(Entity, Entity)>>,
    burn_query: Query<&components::Burn>,
    mut burnable_query: Query<&mut components::Burnable>,
    time: Res<Time>,
) {
    for event in burn_burnable_events.iter() {
        let pair = (event.burn_entity, event.burnable_entity);
        match event.stage {
            events::EventStage::Started => burn_burnable_pairs.insert(pair),
            events::EventStage::Stopped => burn_burnable_pairs.remove(&pair),
        };
    }

    for (burn_entity, burnable_entity) in burn_burnable_pairs.iter() {
        if let (Ok(burn), Ok(mut burnable)) = (
            burn_query.get_component::<components::Burn>(*burn_entity),
            burnable_query.get_component_mut::<components::Burnable>(*burnable_entity),
        ) {
            if !burnable.inactive {
                burnable.resist -= burn.strength * time.delta_seconds();
            }
        }
    }
}
