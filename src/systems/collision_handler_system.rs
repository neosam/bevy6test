use bevy::prelude::*;
use heron::prelude::*;

use crate::{components, events};

macro_rules! check_collision {
    ($t1:ty, $t1_query:expr, $t2:ty, $t2_query:expr, $entity1:expr, $entity2:expr) => {
        if let (Ok(_), Ok(_)) = (
            $t1_query.get_component::<$t1>($entity1),
            $t2_query.get_component::<$t2>($entity2),
        ) {
            Some(($entity1, $entity2))
        } else if let (Ok(_), Ok(_)) = (
            $t2_query.get_component::<$t2>($entity1),
            $t1_query.get_component::<$t1>($entity2),
        ) {
            Some(($entity2, $entity1))
        } else { None }
    };
}

pub fn collision_handler_system(
    mut collision_events: EventReader<CollisionEvent>,
    burn_query: Query<&components::Burn>,
    burnable_query: Query<&components::Burnable>,
    mut burn_burnable_events: EventWriter<events::BurnBurnableEvent>,
) {
    for event in collision_events.iter() {
        println!("{:?}", event);
        let (entity1, entity2) = event.collision_shape_entities();
        if let Some((entity1, entity2)) = check_collision!(
            components::Burn, burn_query, components::Burnable, burnable_query, entity1, entity2)
        {
            burn_burnable_events.send(events::BurnBurnableEvent { 
                burn_entity: entity1,
                burnable_entity: entity2,
                stage: events::EventStage::from_started(event.is_started()),
            });
        }
    }
}