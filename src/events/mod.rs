use bevy::prelude::*;

pub enum InputEvent {
    MoveNorth,
    MoveSouth,
    MoveEast,
    MoveWest,
    Stop,
    Shoot,
}

pub enum EventStage {
    Started,
    Stopped,
}
impl EventStage {
    pub fn from_started(started: bool) -> Self {
        match started {
            true => EventStage::Started,
            false => EventStage::Stopped,
        }
    }
}
pub struct BurnBurnableEvent {
    pub burn_entity: Entity,
    pub burnable_entity: Entity,
    pub stage: EventStage,
}

pub struct DamagerHealthEvent {
    pub damager_entity: Entity,
    pub health_entity: Entity,
    pub stage: EventStage,
}
